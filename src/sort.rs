use {
    crate::style::SortStyle,
    crossterm::{
        cursor,
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
        execute,
        terminal::{self, Clear, ClearType},
    },
    miette::IntoDiagnostic,
    owo_colors::OwoColorize,
    std::io::{Write, stdout},
};

/// a rearrangable list of items
#[derive(Clone)]
pub struct Sort {
    /// the prompt to display on the CLI
    prompt: String,
    /// the choices to be provided
    items: Vec<String>,
    /// the character to show before the prompt message
    prompt_prefix: String,
    /// an optional help message
    help_message: Option<String>,
    /// the number of options to show on each page
    page_size: usize,
    /// whether to show hints
    show_hints: bool,
    show_indices: bool,
    allow_escape: bool,
    vim_mode: bool,
    style: SortStyle,
    validation: Option<fn(&[String]) -> Result<(), String>>,
}

impl Sort {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            items: Vec::new(),
            prompt_prefix: "↕".into(),
            help_message: None,
            page_size: 10,
            show_hints: true,
            show_indices: true,
            allow_escape: true,
            vim_mode: false,
            style: SortStyle::default(),
            validation: None,
        }
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn with_items(mut self, items: Vec<impl Into<String>>) -> Self {
        self.items = items.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn with_item(mut self, item: impl Into<String>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn with_prompt_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prompt_prefix = prefix.into();
        self
    }

    pub fn with_help_message(mut self, message: impl Into<String>) -> Self {
        self.help_message = Some(message.into());
        self
    }

    pub fn with_page_size(mut self, size: usize) -> Self {
        self.page_size = size.max(3);
        self
    }

    pub fn with_hints(mut self, enabled: bool) -> Self {
        self.show_hints = enabled;
        self
    }

    pub fn with_indices(mut self, show: bool) -> Self {
        self.show_indices = show;
        self
    }

    pub fn with_escape(mut self, allow: bool) -> Self {
        self.allow_escape = allow;
        self
    }

    pub fn with_vim_mode(mut self, enabled: bool) -> Self {
        self.vim_mode = enabled;
        self
    }

    pub fn with_style(mut self, style: SortStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_validation(mut self, validation: fn(&[String]) -> Result<(), String>) -> Self {
        self.validation = Some(validation);
        self
    }

    pub fn ask(&self) -> miette::Result<Vec<String>> {
        if self.items.is_empty() {
            return Err(miette::miette!("No items provided"));
        }

        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = terminal::disable_raw_mode();
            std::panic::take_hook()(panic_info);
        }));

        let result = self.ask_internal();

        let _ = std::panic::take_hook();
        std::panic::set_hook(original_hook);

        result
    }

    fn ask_internal(&self) -> miette::Result<Vec<String>> {
        let mut items = self.items.clone();
        let mut cursor = 0usize;
        let mut grabbed = false;
        let mut scroll_offset = 0usize;
        let mut error_message: Option<String> = None;

        terminal::enable_raw_mode().into_diagnostic()?;

        while event::poll(std::time::Duration::from_millis(0)).into_diagnostic()? {
            event::read().into_diagnostic()?;
        }

        let mut last_render_lines = self.render(
            &mut stdout(),
            &items,
            cursor,
            grabbed,
            scroll_offset,
            error_message.as_deref(),
        )?;
        stdout().flush().into_diagnostic()?;

        loop {
            if let Event::Key(key_event) = event::read().into_diagnostic()? {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                error_message = None;

                match self.handle_key(
                    key_event,
                    &mut items,
                    &mut cursor,
                    &mut grabbed,
                    &mut scroll_offset,
                ) {
                    Ok(Some(())) => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        if last_render_lines > 0 {
                            execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                                .into_diagnostic()?;
                        }
                        execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                        execute!(stdout(), Clear(ClearType::FromCursorDown)).into_diagnostic()?;
                        self.show_result(&mut stdout(), &items)?;
                        return Ok(items);
                    }
                    Ok(None) => {}
                    Err(e) => {
                        if e == "Cancelled" {
                            terminal::disable_raw_mode().into_diagnostic()?;
                            if last_render_lines > 0 {
                                execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                                    .into_diagnostic()?;
                            }
                            execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                            execute!(stdout(), Clear(ClearType::FromCursorDown))
                                .into_diagnostic()?;
                            self.show_error(&mut stdout(), &e)?;
                            return Err(miette::miette!(e));
                        }
                        error_message = Some(e);
                    }
                }

                if last_render_lines > 0 {
                    execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                        .into_diagnostic()?;
                }
                execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                execute!(stdout(), Clear(ClearType::FromCursorDown)).into_diagnostic()?;
                last_render_lines = self.render(
                    &mut stdout(),
                    &items,
                    cursor,
                    grabbed,
                    scroll_offset,
                    error_message.as_deref(),
                )?;
                stdout().flush().into_diagnostic()?;
            }
        }
    }

    fn handle_key(
        &self,
        key_event: KeyEvent,
        items: &mut Vec<String>,
        cursor: &mut usize,
        grabbed: &mut bool,
        scroll_offset: &mut usize,
    ) -> Result<Option<()>, String> {
        if key_event.modifiers.contains(KeyModifiers::CONTROL)
            && matches!(key_event.code, KeyCode::Char('c'))
        {
            let _ = terminal::disable_raw_mode();
            std::process::exit(130);
        }

        let total = items.len();

        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') if self.vim_mode || key_event.code == KeyCode::Up => {
                if *grabbed && *cursor > 0 {
                    items.swap(*cursor, *cursor - 1);
                    *cursor -= 1;
                } else if !*grabbed && *cursor > 0 {
                    *cursor -= 1;
                } else if !*grabbed {
                    *cursor = total - 1;
                    *scroll_offset = cursor.saturating_sub(self.page_size - 1);
                }

                if *cursor < *scroll_offset {
                    *scroll_offset = *cursor;
                }
                Ok(None)
            }
            KeyCode::Down | KeyCode::Char('j')
                if self.vim_mode || key_event.code == KeyCode::Down =>
            {
                if *grabbed && *cursor < total - 1 {
                    items.swap(*cursor, *cursor + 1);
                    *cursor += 1;
                } else if !*grabbed && *cursor < total - 1 {
                    *cursor += 1;
                } else if !*grabbed {
                    *cursor = 0;
                    *scroll_offset = 0;
                }

                if *cursor >= *scroll_offset + self.page_size {
                    *scroll_offset = cursor.saturating_sub(self.page_size - 1);
                }
                Ok(None)
            }
            KeyCode::Char(' ') => {
                *grabbed = !*grabbed;
                Ok(None)
            }
            KeyCode::Enter => {
                if *grabbed {
                    *grabbed = false;
                    return Ok(None);
                }

                if let Some(validator) = self.validation {
                    validator(items)?;
                }
                Ok(Some(()))
            }
            KeyCode::Home | KeyCode::Char('g')
                if !*grabbed && (self.vim_mode || key_event.code == KeyCode::Home) =>
            {
                *cursor = 0;
                *scroll_offset = 0;
                Ok(None)
            }
            KeyCode::End | KeyCode::Char('G')
                if !*grabbed && (self.vim_mode || key_event.code == KeyCode::End) =>
            {
                *cursor = total - 1;
                *scroll_offset = cursor.saturating_sub(self.page_size - 1);
                Ok(None)
            }
            KeyCode::Esc if self.allow_escape => {
                if *grabbed {
                    *grabbed = false;
                    Ok(None)
                } else {
                    Err("Cancelled".into())
                }
            }
            _ => Ok(None),
        }
    }

    fn render(
        &self,
        out: &mut std::io::Stdout,
        items: &[String],
        cursor: usize,
        grabbed: bool,
        scroll_offset: usize,
        error: Option<&str>,
    ) -> miette::Result<usize> {
        let mut line_count = 0;

        writeln!(
            out,
            "{} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            self.prompt.style(self.style.prompt),
        )
        .into_diagnostic()?;
        line_count += 1;

        if let Some(ref help) = self.help_message {
            writeln!(out, "  {}", help.style(self.style.hint)).into_diagnostic()?;
            line_count += 1;
        }

        let end_offset = (scroll_offset + self.page_size).min(items.len());
        let visible_items = &items[scroll_offset..end_offset];

        for (i, item) in visible_items.iter().enumerate() {
            let absolute_index = scroll_offset + i;
            let is_cursor = absolute_index == cursor;

            let (marker, item_style) = if is_cursor && grabbed {
                ("≡", self.style.grabbed)
            } else if is_cursor {
                ("▸", self.style.cursor)
            } else {
                (" ", self.style.item)
            };

            if self.show_indices {
                writeln!(
                    out,
                    "  {} {} {}",
                    marker.style(if is_cursor && grabbed {
                        self.style.grabbed
                    } else {
                        self.style.cursor
                    }),
                    format!("{}.", absolute_index + 1).style(self.style.index),
                    item.style(item_style)
                )
                .into_diagnostic()?;
            } else {
                writeln!(
                    out,
                    "  {} {}",
                    marker.style(if is_cursor && grabbed {
                        self.style.grabbed
                    } else {
                        self.style.cursor
                    }),
                    item.style(item_style)
                )
                .into_diagnostic()?;
            }
            line_count += 1;
        }

        if scroll_offset > 0 {
            writeln!(
                out,
                "  {}",
                format!("(↑ {} more above)", scroll_offset).style(self.style.hint)
            )
            .into_diagnostic()?;
            line_count += 1;
        }

        if end_offset < items.len() {
            writeln!(
                out,
                "  {}",
                format!("(↓ {} more below)", items.len() - end_offset).style(self.style.hint)
            )
            .into_diagnostic()?;
            line_count += 1;
        }

        if let Some(err) = error {
            writeln!(
                out,
                "  {} {}",
                "✗".style(self.style.error),
                err.style(self.style.error_hint)
            )
            .into_diagnostic()?;
            line_count += 1;
        }

        if self.show_hints {
            let mut hints = vec![];

            if self.vim_mode {
                hints.push("j/k or ↑↓ to navigate");
            } else {
                hints.push("↑↓ to navigate");
            }

            hints.push("Space to grab/release");
            hints.push("Enter to submit");

            if self.allow_escape {
                hints.push("Esc to cancel/release");
            }

            writeln!(out, "  {}", hints.join(", ").style(self.style.hint)).into_diagnostic()?;
            line_count += 1;
        }

        Ok(line_count)
    }

    fn show_error(&self, out: &mut std::io::Stdout, error: &str) -> miette::Result<()> {
        writeln!(
            out,
            "{} {}",
            "✗".style(self.style.error),
            error.style(self.style.error_hint),
        )
        .into_diagnostic()?;

        Ok(())
    }

    fn show_result(&self, out: &mut std::io::Stdout, items: &[String]) -> miette::Result<()> {
        let result_text = items
            .iter()
            .enumerate()
            .map(|(i, item)| format!("{}. {}", i + 1, item))
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(
            out,
            "{} {} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            self.prompt.style(self.style.prompt),
            result_text.style(self.style.cursor).bold(),
        )
        .into_diagnostic()?;

        out.flush().into_diagnostic()?;
        Ok(())
    }
}
