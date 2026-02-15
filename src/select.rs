use {
    crate::{
        option::AskOption,
        style::SelectStyle,
        util::CursorGuard,
        validation::{Validate, run_validator},
    },
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

#[derive(Clone)]
#[allow(clippy::type_complexity)]
pub struct Select<T: Clone> {
    prompt: String,
    options: Vec<AskOption<T>>,
    default_index: Option<usize>,
    page_size: usize,
    inline: bool,
    prompt_prefix: String,
    help_message: Option<String>,
    show_hints: bool,
    show_descriptions: bool,
    allow_escape: bool,
    vim_mode: bool,
    style: SelectStyle,
    validation: Option<Box<dyn Validate<usize>>>,
    _cursor_guard: CursorGuard,
}

impl<T: Clone> Select<T> {
    pub fn new(prompt: impl Into<String>) -> Self {
        let _cursor_guard = CursorGuard::new().expect("Failed to initialize cursor guard");
        Self {
            prompt: prompt.into(),
            options: Vec::new(),
            default_index: None,
            page_size: 10,
            inline: false,
            prompt_prefix: "?".into(),
            help_message: None,
            show_hints: true,
            show_descriptions: true,
            allow_escape: true,
            vim_mode: false,
            style: SelectStyle::default(),
            validation: None,
            _cursor_guard,
        }
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn options(&self) -> &[AskOption<T>] {
        &self.options
    }

    pub fn with_options(mut self, options: Vec<AskOption<T>>) -> Self {
        self.options = options;
        self
    }

    pub fn with_option(mut self, option: AskOption<T>) -> Self {
        self.options.push(option);
        self
    }

    pub fn with_default(mut self, index: usize) -> Self {
        self.default_index = Some(index);
        self
    }

    pub fn with_page_size(mut self, size: usize) -> Self {
        self.page_size = size.max(3);
        self
    }

    pub fn with_inline(mut self, inline: bool) -> Self {
        self.inline = inline;
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

    pub fn with_hints(mut self, enabled: bool) -> Self {
        self.show_hints = enabled;
        self
    }

    pub fn with_descriptions(mut self, enabled: bool) -> Self {
        self.show_descriptions = enabled;
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

    pub fn with_style(mut self, style: SelectStyle) -> Self {
        self.style = style;
        self
    }

    /// Register a validator that receives the selected index.
    ///
    /// Accepts any `impl Validate<usize>`, including closures:
    ///
    /// ```rust,ignore
    /// Select::new("Pick")
    ///     .with_validation(|idx: &usize| {
    ///         if *idx == 0 {
    ///             Ok(Validation::Invalid("Pick something else".into()))
    ///         } else {
    ///             Ok(Validation::Valid)
    ///         }
    ///     })
    /// ```
    pub fn with_validation(mut self, validation: impl Validate<usize> + 'static) -> Self {
        self.validation = Some(Box::new(validation));
        self
    }

    pub fn ask(&self) -> miette::Result<AskOption<T>> {
        if self.options.is_empty() {
            return Err(miette::miette!("No options provided"));
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

    fn ask_internal(&self) -> miette::Result<AskOption<T>> {
        let default_index = self.default_index.unwrap_or(0).min(self.options.len() - 1);

        let mut selected = default_index;
        let mut scroll_offset = 0;
        let mut last_render_lines = self.render(&mut stdout(), selected, scroll_offset)?;

        terminal::enable_raw_mode().into_diagnostic()?;

        while event::poll(std::time::Duration::from_millis(0)).into_diagnostic()? {
            event::read().into_diagnostic()?;
        }

        stdout().flush().into_diagnostic()?;

        loop {
            if let Event::Key(key_event) = event::read().into_diagnostic()? {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                match self.handle_key(key_event, &mut selected, &mut scroll_offset) {
                    Ok(Some(())) => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        if last_render_lines > 0 {
                            execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                                .into_diagnostic()?;
                        }
                        execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                        execute!(stdout(), Clear(ClearType::FromCursorDown)).into_diagnostic()?;

                        let selected_option = &self.options[selected];
                        self.show_result(&mut stdout(), selected_option)?;
                        return Ok(selected_option.clone());
                    }
                    Ok(None) => {
                        if last_render_lines > 0 {
                            execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                                .into_diagnostic()?;
                        }
                        execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                        execute!(stdout(), Clear(ClearType::FromCursorDown)).into_diagnostic()?;
                        last_render_lines = self.render(&mut stdout(), selected, scroll_offset)?;
                        stdout().flush().into_diagnostic()?;
                    }
                    Err(e) => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        if last_render_lines > 0 {
                            execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                                .into_diagnostic()?;
                        }
                        execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                        execute!(stdout(), Clear(ClearType::FromCursorDown)).into_diagnostic()?;
                        self.show_error(&mut stdout(), &e)?;
                        stdout().flush().into_diagnostic()?;
                        return Err(miette::miette!(e));
                    }
                }
            }

            stdout().flush().into_diagnostic()?;
        }
    }

    fn handle_key(
        &self,
        key_event: KeyEvent,
        selected: &mut usize,
        scroll_offset: &mut usize,
    ) -> Result<Option<()>, String> {
        if key_event.modifiers.contains(KeyModifiers::CONTROL)
            && matches!(key_event.code, KeyCode::Char('c'))
        {
            let _ = terminal::disable_raw_mode();
            std::process::exit(130);
        }

        let total_options = self.options.len();

        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') if self.vim_mode || key_event.code == KeyCode::Up => {
                if *selected > 0 {
                    *selected -= 1;
                    if *selected < *scroll_offset {
                        *scroll_offset = *selected;
                    }
                } else {
                    *selected = total_options - 1;
                    *scroll_offset = selected.saturating_sub(self.page_size - 1);
                }
                Ok(None)
            }
            KeyCode::Down | KeyCode::Char('j')
                if self.vim_mode || key_event.code == KeyCode::Down =>
            {
                if *selected < total_options - 1 {
                    *selected += 1;
                    if *selected >= *scroll_offset + self.page_size {
                        *scroll_offset = selected.saturating_sub(self.page_size - 1);
                    }
                } else {
                    *selected = 0;
                    *scroll_offset = 0;
                }
                Ok(None)
            }
            KeyCode::Home | KeyCode::Char('g')
                if self.vim_mode || key_event.code == KeyCode::Home =>
            {
                *selected = 0;
                *scroll_offset = 0;
                Ok(None)
            }
            KeyCode::End | KeyCode::Char('G')
                if self.vim_mode || key_event.code == KeyCode::End =>
            {
                *selected = total_options - 1;
                *scroll_offset = selected.saturating_sub(self.page_size - 1);
                Ok(None)
            }
            KeyCode::PageUp => {
                *selected = selected.saturating_sub(self.page_size);
                *scroll_offset = scroll_offset.saturating_sub(self.page_size);
                Ok(None)
            }
            KeyCode::PageDown => {
                *selected = (*selected + self.page_size).min(total_options - 1);
                if *selected >= *scroll_offset + self.page_size {
                    *scroll_offset = selected.saturating_sub(self.page_size - 1);
                }
                Ok(None)
            }
            KeyCode::Enter | KeyCode::Char(' ') => self.validate_and_return(*selected),
            KeyCode::Esc if self.allow_escape => Err("Cancelled".into()),
            _ => Ok(None),
        }
    }

    fn validate_and_return(&self, index: usize) -> Result<Option<()>, String> {
        if let Some(ref validator) = self.validation {
            run_validator(validator.as_ref(), &index)?;
        }
        Ok(Some(()))
    }

    fn render(
        &self,
        out: &mut std::io::Stdout,
        selected: usize,
        scroll_offset: usize,
    ) -> miette::Result<usize> {
        let tw = crate::util::term_width();
        let mut line_count = 0;

        if self.inline {
            write!(
                out,
                "{} {} ",
                self.prompt_prefix.style(self.style.prompt_prefix),
                self.prompt.style(self.style.prompt),
            )
            .into_diagnostic()?;
        } else {
            let line = format!(
                "{} {}",
                self.prompt_prefix.style(self.style.prompt_prefix),
                self.prompt.style(self.style.prompt),
            );
            line_count += crate::util::writeln_physical(out, &line, tw)?;
        }

        if let Some(ref help) = self.help_message {
            let line = format!("  {}", help.style(self.style.hint));
            line_count += crate::util::writeln_physical(out, &line, tw)?;
        }

        let end_offset = (scroll_offset + self.page_size).min(self.options.len());
        let visible_options = &self.options[scroll_offset..end_offset];

        for (i, option) in visible_options.iter().enumerate() {
            let absolute_index = scroll_offset + i;
            let is_selected = absolute_index == selected;

            let marker = if is_selected { "▸" } else { " " };

            if self.show_descriptions && !option.description.is_empty() {
                let name_style = if is_selected {
                    self.style.selected
                } else {
                    self.style.option_name
                };
                let desc_style = if is_selected {
                    self.style.selected_description
                } else {
                    self.style.option_description
                };

                let line = format!(
                    "  {} {}",
                    marker.style(self.style.selected),
                    option.name.style(name_style)
                );
                line_count += crate::util::writeln_physical(out, &line, tw)?;

                let line = format!("      {}", option.description.style(desc_style));
                line_count += crate::util::writeln_physical(out, &line, tw)?;
            } else {
                let style = if is_selected {
                    self.style.selected
                } else {
                    self.style.option_name
                };

                let line = format!(
                    "  {} {}",
                    marker.style(self.style.selected),
                    option.name.style(style)
                );
                line_count += crate::util::writeln_physical(out, &line, tw)?;
            }
        }

        if scroll_offset > 0 {
            let line = format!(
                "  {}",
                format!("(↑ {} more above)", scroll_offset).style(self.style.hint)
            );
            line_count += crate::util::writeln_physical(out, &line, tw)?;
        }

        if end_offset < self.options.len() {
            let line = format!(
                "  {}",
                format!("(↓ {} more below)", self.options.len() - end_offset)
                    .style(self.style.hint)
            );
            line_count += crate::util::writeln_physical(out, &line, tw)?;
        }

        if self.show_hints {
            let mut hints = vec![];

            if self.vim_mode {
                hints.push("j/k or ↑↓ to navigate");
            } else {
                hints.push("↑↓ to navigate");
            }

            if self.options.len() > self.page_size {
                hints.push("PgUp/PgDn to scroll");
            }

            hints.push("Enter to select");

            if self.allow_escape {
                hints.push("Esc to cancel");
            }

            let line = format!("  {}", hints.join(", ").style(self.style.hint));
            line_count += crate::util::writeln_physical(out, &line, tw)?;
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

    fn show_result(&self, out: &mut std::io::Stdout, option: &AskOption<T>) -> miette::Result<()> {
        writeln!(
            out,
            "{} {} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            self.prompt.style(self.style.prompt),
            option.name.style(self.style.selected).bold(),
        )
        .into_diagnostic()?;

        out.flush().into_diagnostic()?;
        Ok(())
    }
}
