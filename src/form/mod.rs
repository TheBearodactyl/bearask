use {
    crate::style::FormStyle,
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
        terminal::{self},
    },
    fields::*,
    miette::IntoDiagnostic,
    owo_colors::OwoColorize,
    result::FormResult,
    std::io::{Write, stdout},
    value::FormValue,
};

pub mod fields;
pub mod key;
pub mod render;
pub mod result;
pub mod value;

struct FormSection {
    title: String,
}

pub struct Form {
    title: String,
    sections: Vec<FormSection>,
    fields: Vec<InternalField>,
    current_section: Option<usize>,
    show_title: bool,
    show_summary: bool,
    allow_escape: bool,
    style: FormStyle,
    validation: Option<fn(&FormResult) -> Result<(), String>>,
}

impl Form {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            sections: Vec::new(),
            fields: Vec::new(),
            current_section: None,
            show_title: true,
            show_summary: true,
            allow_escape: true,
            style: FormStyle::default(),
            validation: None,
        }
    }

    pub fn section(mut self, title: impl Into<String>) -> Self {
        self.sections.push(FormSection {
            title: title.into(),
        });
        self.current_section = Some(self.sections.len() - 1);
        self
    }

    pub fn field(mut self, key: impl Into<String>, builder: impl IntoFormField) -> Self {
        self.fields.push(InternalField {
            key: key.into(),
            section_idx: self.current_section,
            kind: builder.into_field_kind(),
            confirmed: false,
            error: None,
        });
        self
    }

    pub fn with_title(mut self, show: bool) -> Self {
        self.show_title = show;
        self
    }

    pub fn with_summary(mut self, show: bool) -> Self {
        self.show_summary = show;
        self
    }

    pub fn with_escape(mut self, allow: bool) -> Self {
        self.allow_escape = allow;
        self
    }

    pub fn with_style(mut self, style: FormStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_validation(mut self, validation: fn(&FormResult) -> Result<(), String>) -> Self {
        self.validation = Some(validation);
        self
    }

    pub fn ask(mut self) -> miette::Result<FormResult> {
        if self.fields.is_empty() {
            return Err(miette::miette!("No fields in form"));
        }

        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = terminal::disable_raw_mode();
            std::panic::take_hook()(panic_info);
        }));

        let result = self.run_event_loop();

        let _ = std::panic::take_hook();
        std::panic::set_hook(original_hook);

        result
    }

    fn run_event_loop(&mut self) -> miette::Result<FormResult> {
        let mut out = stdout();
        let mut active: usize = 0;

        terminal::enable_raw_mode().into_diagnostic()?;

        while event::poll(std::time::Duration::from_millis(0)).into_diagnostic()? {
            event::read().into_diagnostic()?;
        }

        let mut last_lines = self.render(&mut out, active)?;
        out.flush().into_diagnostic()?;

        loop {
            if let Event::Key(key_event) = event::read().into_diagnostic()? {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                if key_event.modifiers.contains(KeyModifiers::CONTROL)
                    && matches!(key_event.code, KeyCode::Char('c'))
                {
                    let _ = terminal::disable_raw_mode();
                    std::process::exit(130);
                }

                let action = self.dispatch_key(key_event, active);

                match action {
                    FieldAction::Continue => {}
                    FieldAction::Confirm(_) => {
                        self.fields[active].confirmed = true;
                        self.fields[active].error = None;

                        if self.fields.iter().all(|f| f.confirmed) {
                            let entries: Vec<(String, FormValue)> = self
                                .fields
                                .iter()
                                .map(|f| (f.key.clone(), f.kind.current_value()))
                                .collect();
                            let result = FormResult { entries };

                            if let Some(validator) = self.validation
                                && let Err(e) = validator(&result)
                            {
                                self.fields[active].error = Some(e);

                                Self::clear_render(&mut out, last_lines)?;
                                last_lines = self.render(&mut out, active)?;
                                out.flush().into_diagnostic()?;
                                continue;
                            }

                            terminal::disable_raw_mode().into_diagnostic()?;
                            Self::clear_render(&mut out, last_lines)?;
                            self.render_final(&mut out)?;
                            out.flush().into_diagnostic()?;
                            return Ok(result);
                        }

                        active = self.next_unconfirmed(active);
                    }
                    FieldAction::Next => {
                        active = (active + 1) % self.fields.len();
                    }
                    FieldAction::Prev => {
                        active = if active == 0 {
                            self.fields.len() - 1
                        } else {
                            active - 1
                        };
                    }
                    FieldAction::Cancel => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        Self::clear_render(&mut out, last_lines)?;
                        writeln!(
                            out,
                            "{} {}",
                            "✗".style(self.style.error),
                            "Cancelled".style(self.style.error_hint),
                        )
                        .into_diagnostic()?;
                        out.flush().into_diagnostic()?;
                        return Err(miette::miette!("Cancelled"));
                    }
                }

                Self::clear_render(&mut out, last_lines)?;
                last_lines = self.render(&mut out, active)?;
                out.flush().into_diagnostic()?;
            }
        }
    }

    fn next_unconfirmed(&self, from: usize) -> usize {
        let len = self.fields.len();
        for offset in 1..=len {
            let idx = (from + offset) % len;
            if !self.fields[idx].confirmed {
                return idx;
            }
        }

        from
    }

    fn adjust_scroll(cursor: usize, scroll_offset: &mut usize, page_size: usize, _total: usize) {
        if cursor < *scroll_offset {
            *scroll_offset = cursor;
        }
        if cursor >= *scroll_offset + page_size {
            *scroll_offset = cursor.saturating_sub(page_size - 1);
        }
    }
}

impl From<FormValue> for FieldAction {
    fn from(v: FormValue) -> Self {
        FieldAction::Confirm(v)
    }
}
