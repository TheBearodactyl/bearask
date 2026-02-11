use {
    crate::style::PasswordStyle,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PasswordDisplayMode {
    Masked,
    Hidden,
    Full,
}

#[derive(Clone)]
pub struct Password {
    prompt: String,
    prompt_prefix: String,
    help_message: Option<String>,
    mask_char: char,
    display_mode: PasswordDisplayMode,
    allow_toggle: bool,
    show_strength: bool,
    min_length: Option<usize>,
    max_length: Option<usize>,
    allow_escape: bool,
    show_hints: bool,
    confirmation: Option<String>,
    style: PasswordStyle,
    validation: Option<fn(&str) -> Result<(), String>>,
}

impl Password {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            prompt_prefix: "🔒".into(),
            help_message: None,
            mask_char: '●',
            display_mode: PasswordDisplayMode::Masked,
            allow_toggle: true,
            show_strength: false,
            min_length: None,
            max_length: None,
            allow_escape: true,
            show_hints: true,
            confirmation: None,
            style: PasswordStyle::default(),
            validation: None,
        }
    }

    pub fn with_prompt_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prompt_prefix = prefix.into();
        self
    }

    pub fn with_help_message(mut self, message: impl Into<String>) -> Self {
        self.help_message = Some(message.into());
        self
    }

    pub fn with_mask_char(mut self, c: char) -> Self {
        self.mask_char = c;
        self
    }

    pub fn with_display_mode(mut self, mode: PasswordDisplayMode) -> Self {
        self.display_mode = mode;
        self
    }

    pub fn with_toggle(mut self, allow: bool) -> Self {
        self.allow_toggle = allow;
        self
    }

    pub fn with_strength_indicator(mut self, show: bool) -> Self {
        self.show_strength = show;
        self
    }

    pub fn with_min_length(mut self, min: usize) -> Self {
        self.min_length = Some(min);
        self
    }

    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    pub fn with_escape(mut self, allow: bool) -> Self {
        self.allow_escape = allow;
        self
    }

    pub fn with_hints(mut self, enabled: bool) -> Self {
        self.show_hints = enabled;
        self
    }

    pub fn with_confirmation(mut self, prompt: impl Into<String>) -> Self {
        self.confirmation = Some(prompt.into());
        self
    }

    pub fn with_style(mut self, style: PasswordStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_validation(mut self, validation: fn(&str) -> Result<(), String>) -> Self {
        self.validation = Some(validation);
        self
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn ask(&self) -> miette::Result<String> {
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

    fn ask_internal(&self) -> miette::Result<String> {
        let password = self.ask_single(&self.prompt)?;

        if let Some(ref confirm_prompt) = self.confirmation {
            let confirmed = self.ask_single(confirm_prompt)?;
            if password != confirmed {
                return Err(miette::miette!("Passwords do not match"));
            }
        }

        Ok(password)
    }

    fn ask_single(&self, prompt: &str) -> miette::Result<String> {
        let mut input = String::new();
        let mut cursor_pos: usize = 0;
        let mut revealed = self.display_mode == PasswordDisplayMode::Full;
        let mut error_message: Option<String> = None;

        terminal::enable_raw_mode().into_diagnostic()?;

        while event::poll(std::time::Duration::from_millis(0)).into_diagnostic()? {
            event::read().into_diagnostic()?;
        }

        let mut last_render_lines =
            self.render(&mut stdout(), prompt, &input, revealed, error_message.as_deref())?;
        stdout().flush().into_diagnostic()?;

        loop {
            if let Event::Key(key_event) = event::read().into_diagnostic()? {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                error_message = None;

                match self.handle_key(key_event, &mut input, &mut cursor_pos, &mut revealed) {
                    Ok(Some(answer)) => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        if last_render_lines > 0 {
                            execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                                .into_diagnostic()?;
                        }
                        execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                        execute!(stdout(), Clear(ClearType::FromCursorDown)).into_diagnostic()?;
                        self.show_result(&mut stdout(), prompt)?;
                        return Ok(answer);
                    }
                    Ok(None) => {}
                    Err(e) if e == "Cancelled" => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        if last_render_lines > 0 {
                            execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                                .into_diagnostic()?;
                        }
                        execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                        execute!(stdout(), Clear(ClearType::FromCursorDown)).into_diagnostic()?;
                        return Err(miette::miette!("Cancelled"));
                    }
                    Err(e) => {
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
                    prompt,
                    &input,
                    revealed,
                    error_message.as_deref(),
                )?;
                stdout().flush().into_diagnostic()?;
            }
        }
    }

    fn handle_key(
        &self,
        key_event: KeyEvent,
        input: &mut String,
        cursor_pos: &mut usize,
        revealed: &mut bool,
    ) -> Result<Option<String>, String> {
        if key_event.modifiers.contains(KeyModifiers::CONTROL)
            && matches!(key_event.code, KeyCode::Char('c'))
        {
            let _ = terminal::disable_raw_mode();
            std::process::exit(130);
        }

        if key_event.modifiers.contains(KeyModifiers::CONTROL)
            && matches!(key_event.code, KeyCode::Char('r'))
            && self.allow_toggle
            && self.display_mode == PasswordDisplayMode::Masked
        {
            *revealed = !*revealed;
            return Ok(None);
        }

        match key_event.code {
            KeyCode::Enter => {
                if let Some(min) = self.min_length {
                    if input.len() < min {
                        return Err(format!("Must be at least {} characters", min));
                    }
                }
                if let Some(max) = self.max_length {
                    if input.len() > max {
                        return Err(format!("Must be at most {} characters", max));
                    }
                }
                if let Some(validator) = self.validation {
                    validator(input)?;
                }
                Ok(Some(input.clone()))
            }
            KeyCode::Char(c) => {
                input.insert(*cursor_pos, c);
                *cursor_pos += 1;
                Ok(None)
            }
            KeyCode::Backspace if *cursor_pos > 0 => {
                *cursor_pos -= 1;
                input.remove(*cursor_pos);
                Ok(None)
            }
            KeyCode::Delete if *cursor_pos < input.len() => {
                input.remove(*cursor_pos);
                Ok(None)
            }
            KeyCode::Left if *cursor_pos > 0 => {
                *cursor_pos -= 1;
                Ok(None)
            }
            KeyCode::Right if *cursor_pos < input.len() => {
                *cursor_pos += 1;
                Ok(None)
            }
            KeyCode::Home => {
                *cursor_pos = 0;
                Ok(None)
            }
            KeyCode::End => {
                *cursor_pos = input.len();
                Ok(None)
            }
            KeyCode::Esc if self.allow_escape => Err("Cancelled".into()),
            _ => Ok(None),
        }
    }

    fn password_strength(input: &str) -> (&str, usize) {
        let len = input.len();
        let has_upper = input.chars().any(|c| c.is_uppercase());
        let has_lower = input.chars().any(|c| c.is_lowercase());
        let has_digit = input.chars().any(|c| c.is_ascii_digit());
        let has_special = input.chars().any(|c| !c.is_alphanumeric());

        let mut score = 0;
        if len >= 8 {
            score += 1;
        }
        if len >= 12 {
            score += 1;
        }
        if has_upper && has_lower {
            score += 1;
        }
        if has_digit {
            score += 1;
        }
        if has_special {
            score += 1;
        }

        match score {
            0..=1 => ("weak", 1),
            2..=3 => ("medium", 2),
            _ => ("strong", 3),
        }
    }

    fn render(
        &self,
        out: &mut std::io::Stdout,
        prompt: &str,
        input: &str,
        revealed: bool,
        error: Option<&str>,
    ) -> miette::Result<usize> {
        let mut line_count = 0;

        writeln!(
            out,
            "{} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            prompt.style(self.style.prompt),
        )
        .into_diagnostic()?;
        line_count += 1;

        if let Some(ref help) = self.help_message {
            writeln!(out, "  {}", help.style(self.style.hint)).into_diagnostic()?;
            line_count += 1;
        }

        let display_text = if input.is_empty() {
            " ".to_string()
        } else if revealed {
            input.to_string()
        } else {
            match self.display_mode {
                PasswordDisplayMode::Hidden => format!("[{} chars]", input.len()),
                PasswordDisplayMode::Masked | PasswordDisplayMode::Full => {
                    self.mask_char.to_string().repeat(input.len())
                }
            }
        };

        let input_style = if revealed {
            self.style.input_revealed
        } else {
            self.style.input_masked
        };

        writeln!(out, "  {}", display_text.style(input_style)).into_diagnostic()?;
        line_count += 1;

        if self.show_strength && !input.is_empty() {
            let (label, level) = Self::password_strength(input);
            let bar_width = 20;
            let filled = (bar_width * level) / 3;
            let empty = bar_width - filled;
            let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

            let strength_style = match level {
                1 => self.style.strength_weak,
                2 => self.style.strength_medium,
                _ => self.style.strength_strong,
            };

            writeln!(
                out,
                "  {} {}",
                bar.style(strength_style),
                label.style(strength_style)
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
            hints.push("Enter to submit");
            if self.allow_toggle && self.display_mode == PasswordDisplayMode::Masked {
                hints.push("Ctrl+R to reveal/hide");
            }
            if self.allow_escape {
                hints.push("Esc to cancel");
            }

            writeln!(out, "  {}", hints.join(", ").style(self.style.hint)).into_diagnostic()?;
            line_count += 1;
        }

        Ok(line_count)
    }

    fn show_result(&self, out: &mut std::io::Stdout, prompt: &str) -> miette::Result<()> {
        writeln!(
            out,
            "{} {} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            prompt.style(self.style.prompt),
            "●●●●●●●●".style(self.style.input_masked).bold(),
        )
        .into_diagnostic()?;

        out.flush().into_diagnostic()?;
        Ok(())
    }
}
