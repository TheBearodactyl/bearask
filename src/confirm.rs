use {
    crate::{
        style::ConfirmStyle,
        util::CursorGuard,
        validation::{Validate, run_validator},
    },
    crossterm::{
        ExecutableCommand, cursor,
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
        terminal::{self, Clear, ClearType},
    },
    miette::IntoDiagnostic,
    owo_colors::OwoColorize,
    std::io::{Write, stdout},
};

#[derive(Clone, Debug, PartialEq)]
pub enum ConfirmMode {
    TextInput,
    Interactive,
}

#[derive(Clone)]
pub struct Confirm {
    prompt: String,
    default: bool,
    inline: bool,
    mode: ConfirmMode,
    prompt_prefix: String,
    prompt_suffix: Option<String>,
    yes_text: String,
    no_text: String,
    show_hints: bool,
    show_error_hint: bool,
    show_confirmation: bool,
    allow_escape: bool,
    style: ConfirmStyle,
    _cursor_guard: CursorGuard,
    validation: Option<Box<dyn Validate<bool>>>,
}

impl Confirm {
    pub fn new(prompt: impl Into<String>) -> Self {
        let cursor_guard = CursorGuard::new().expect("Failed to initialize cursor guard");

        Self {
            prompt: prompt.into(),
            default: true,
            inline: false,
            mode: ConfirmMode::TextInput,
            prompt_prefix: "?".into(),
            prompt_suffix: None,
            yes_text: "yes".into(),
            no_text: "no".into(),
            show_hints: true,
            show_error_hint: true,
            show_confirmation: true,
            allow_escape: true,
            style: ConfirmStyle::default(),
            _cursor_guard: cursor_guard,
            validation: None,
        }
    }

    pub fn with_default(mut self, default: bool) -> Self {
        self.default = default;
        self
    }

    pub fn with_inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }

    pub fn with_mode(mut self, mode: ConfirmMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_prompt_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prompt_prefix = prefix.into();
        self
    }

    pub fn with_prompt_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.prompt_suffix = Some(suffix.into());
        self
    }

    pub fn with_yes_text(mut self, text: impl Into<String>) -> Self {
        self.yes_text = text.into();
        self
    }

    pub fn with_no_text(mut self, text: impl Into<String>) -> Self {
        self.no_text = text.into();
        self
    }

    pub fn with_hints(mut self, enabled: bool) -> Self {
        self.show_hints = enabled;
        self
    }

    pub fn with_error_hint(mut self, enabled: bool) -> Self {
        self.show_error_hint = enabled;
        self
    }

    pub fn with_confirmation(mut self, enabled: bool) -> Self {
        self.show_confirmation = enabled;
        self
    }

    pub fn with_escape(mut self, allow: bool) -> Self {
        self.allow_escape = allow;
        self
    }

    pub fn with_style(mut self, style: ConfirmStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_validation(mut self, validation: impl Validate<bool> + 'static) -> Self {
        self.validation = Some(Box::new(validation));
        self
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn ask(&self) -> miette::Result<bool> {
        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = terminal::disable_raw_mode();
            std::panic::take_hook()(panic_info);
        }));

        let result = match self.mode {
            ConfirmMode::TextInput => self.ask_text_input(),
            ConfirmMode::Interactive => self.ask_interactive(),
        };

        let _ = std::panic::take_hook();
        std::panic::set_hook(original_hook);

        result
    }

    fn ask_interactive(&self) -> miette::Result<bool> {
        let mut out = stdout();
        let mut selected = self.default;

        terminal::enable_raw_mode().into_diagnostic()?;

        while event::poll(std::time::Duration::from_millis(0)).into_diagnostic()? {
            event::read().into_diagnostic()?;
        }

        out.execute(cursor::SavePosition).into_diagnostic()?;
        out.flush().into_diagnostic()?;

        self.render_interactive_prompt(&mut out, selected)?;
        out.flush().into_diagnostic()?;

        loop {
            if let Event::Key(key_event) = event::read().into_diagnostic()? {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                match self.handle_interactive_key(key_event, &mut selected) {
                    Ok(Some(answer)) => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        if self.show_confirmation {
                            out.execute(cursor::RestorePosition).into_diagnostic()?;
                            out.execute(Clear(ClearType::FromCursorDown))
                                .into_diagnostic()?;
                            self.show_result(&mut out, answer)?;
                        }
                        return Ok(answer);
                    }
                    Ok(None) => {
                        out.execute(cursor::RestorePosition).into_diagnostic()?;
                        out.execute(Clear(ClearType::FromCursorDown))
                            .into_diagnostic()?;
                        self.render_interactive_prompt(&mut out, selected)?;
                        out.flush().into_diagnostic()?;
                    }
                    Err(e) => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        out.execute(cursor::RestorePosition).into_diagnostic()?;
                        out.execute(Clear(ClearType::FromCursorDown))
                            .into_diagnostic()?;
                        self.show_error(&mut out, &e)?;
                        out.flush().into_diagnostic()?;
                        return Err(miette::miette!(e));
                    }
                }
            }
        }
    }

    fn ask_text_input(&self) -> miette::Result<bool> {
        let mut out = stdout();
        terminal::enable_raw_mode().into_diagnostic()?;

        while event::poll(std::time::Duration::from_millis(0)).into_diagnostic()? {
            event::read().into_diagnostic()?;
        }

        out.execute(cursor::SavePosition).into_diagnostic()?;
        out.flush().into_diagnostic()?;

        self.render_prompt(&mut out)?;
        out.flush().into_diagnostic()?;

        loop {
            if let Event::Key(key_event) = event::read().into_diagnostic()? {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                let result = self.handle_text_key(key_event, &mut out);

                match result {
                    Ok(Some(answer)) => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        if self.show_confirmation {
                            out.execute(cursor::RestorePosition).into_diagnostic()?;
                            out.execute(Clear(ClearType::FromCursorDown))
                                .into_diagnostic()?;
                            self.show_result(&mut out, answer)?;
                        }
                        return Ok(answer);
                    }
                    Ok(None) => {
                        continue;
                    }
                    Err(e) => {
                        out.execute(cursor::RestorePosition).into_diagnostic()?;
                        out.execute(Clear(ClearType::FromCursorDown))
                            .into_diagnostic()?;
                        self.show_error(&mut out, &e)?;
                        out.flush().into_diagnostic()?;
                        self.render_prompt(&mut out)?;
                        out.flush().into_diagnostic()?;
                    }
                }
            }
        }
    }

    fn handle_text_key(
        &self,
        key_event: KeyEvent,
        _out: &mut std::io::Stdout,
    ) -> Result<Option<bool>, String> {
        if key_event.modifiers.contains(KeyModifiers::CONTROL)
            && matches!(key_event.code, KeyCode::Char('c'))
        {
            let _ = terminal::disable_raw_mode();
            std::process::exit(130);
        }

        match key_event.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => self.validate_and_return(true),
            KeyCode::Char('n') | KeyCode::Char('N') => self.validate_and_return(false),
            KeyCode::Char('1') | KeyCode::Char('t') | KeyCode::Char('T') => {
                self.validate_and_return(true)
            }
            KeyCode::Char('0') | KeyCode::Char('f') | KeyCode::Char('F') => {
                self.validate_and_return(false)
            }
            KeyCode::Enter => self.validate_and_return(self.default),
            KeyCode::Esc if self.allow_escape => Err("Cancelled".into()),
            _ => Err("Invalid input. Expected: y/n, 1/0, t/f, or Enter for default".to_string()),
        }
    }

    fn handle_interactive_key(
        &self,
        key_event: KeyEvent,
        selected: &mut bool,
    ) -> Result<Option<bool>, String> {
        if key_event.modifiers.contains(KeyModifiers::CONTROL)
            && matches!(key_event.code, KeyCode::Char('c'))
        {
            let _ = terminal::disable_raw_mode();
            std::process::exit(130);
        }

        match key_event.code {
            KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                *selected = !*selected;
                Ok(None)
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                *selected = true;
                Ok(None)
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                *selected = false;
                Ok(None)
            }
            KeyCode::Enter | KeyCode::Char(' ') => self.validate_and_return(*selected),
            KeyCode::Esc if self.allow_escape => Err("Cancelled".into()),
            _ => Ok(None),
        }
    }

    fn validate_and_return(&self, value: bool) -> Result<Option<bool>, String> {
        if let Some(ref validator) = self.validation {
            run_validator(validator.as_ref(), &value)?;
        }
        Ok(Some(value))
    }

    fn render_prompt(&self, out: &mut std::io::Stdout) -> miette::Result<()> {
        let tw = crate::util::term_width();

        if self.inline {
            let line = format!(
                "{} {} ",
                self.prompt_prefix.style(self.style.prompt_prefix),
                self.prompt.style(self.style.prompt),
            );
            write!(out, "{}", line).into_diagnostic()?;
        } else {
            let line = format!(
                "{} {}",
                self.prompt_prefix.style(self.style.prompt_prefix),
                self.prompt.style(self.style.prompt),
            );
            crate::util::writeln_physical(out, &line, tw)?;
        }

        if let Some(suffix) = &self.prompt_suffix {
            let line = format!("{} ", suffix.style(self.style.hint));
            write!(out, "{}", line).into_diagnostic()?;
        }

        if self.show_hints {
            let default_hint = if self.default {
                &self.yes_text
            } else {
                &self.no_text
            };
            let line = format!(
                "({}/{}, default: {}) ",
                self.yes_text.style(self.style.yes_style),
                self.no_text.style(self.style.no_style),
                default_hint.style(self.style.default_value),
            );
            write!(out, "{}", line).into_diagnostic()?;
        }

        Ok(())
    }

    fn render_interactive_prompt(
        &self,
        out: &mut std::io::Stdout,
        selected: bool,
    ) -> miette::Result<()> {
        let tw = crate::util::term_width();

        if self.inline {
            let line = format!(
                "{} {} ",
                self.prompt_prefix.style(self.style.prompt_prefix),
                self.prompt.style(self.style.prompt),
            );
            write!(out, "{}", line).into_diagnostic()?;
        } else {
            let line = format!(
                "{} {}",
                self.prompt_prefix.style(self.style.prompt_prefix),
                self.prompt.style(self.style.prompt),
            );
            crate::util::writeln_physical(out, &line, tw)?;
        }

        let options_line = if selected {
            format!(
                "  {}  {}",
                format!("▸ {}", self.yes_text).style(self.style.selected),
                format!("  {}", self.no_text).style(self.style.no_style)
            )
        } else {
            format!(
                "  {}  {}",
                format!("  {}", self.yes_text).style(self.style.yes_style),
                format!("▸ {}", self.no_text).style(self.style.selected)
            )
        };
        crate::util::writeln_physical(out, &options_line, tw)?;

        if self.show_hints {
            let hint_line = format!(
                "  {}",
                "← → to select, Enter to confirm, Esc to cancel".style(self.style.hint)
            );
            crate::util::writeln_physical(out, &hint_line, tw)?;
        }

        Ok(())
    }

    fn show_error(&self, out: &mut std::io::Stdout, error: &str) -> miette::Result<()> {
        if self.show_error_hint {
            let tw = crate::util::term_width();
            let line = format!(
                "{} {}",
                self.style
                    .error_prefix
                    .as_deref()
                    .unwrap_or("✗")
                    .style(self.style.error),
                error.style(self.style.error_hint),
            );
            crate::util::writeln_physical(out, &line, tw)?;
        }

        Ok(())
    }

    fn show_result(&self, out: &mut std::io::Stdout, answer: bool) -> miette::Result<()> {
        let result_text = if answer {
            &self.yes_text
        } else {
            &self.no_text
        };
        let result_style = if answer {
            self.style.yes_style
        } else {
            self.style.no_style
        };

        let tw = crate::util::term_width();
        let line = format!(
            "{} {} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            self.prompt.style(self.style.prompt),
            result_text.style(result_style).bold(),
        );
        crate::util::writeln_physical(out, &line, tw)?;

        out.flush().into_diagnostic()?;
        Ok(())
    }
}
