use {
    crate::{
        style::TextInputStyle,
        validation::{Validate, run_validator},
    },
    crossterm::{
        cursor,
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
        execute,
        terminal::{self, Clear, ClearType},
    },
    dyn_clone::DynClone,
    miette::IntoDiagnostic,
    owo_colors::OwoColorize,
    std::io::{Write, stdout},
};

pub type Replacement = Option<String>;

pub trait Autocomplete: DynClone {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, String>;

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, String>;
}

dyn_clone::clone_trait_object!(Autocomplete);

#[derive(Clone)]
pub struct SimpleAutocomplete {
    options: Vec<String>,
}

impl SimpleAutocomplete {
    pub fn new(options: Vec<String>) -> Self {
        Self { options }
    }
}

impl Autocomplete for SimpleAutocomplete {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, String> {
        if input.is_empty() {
            return Ok(self.options.clone());
        }

        Ok(self
            .options
            .iter()
            .filter(|opt| opt.to_lowercase().starts_with(&input.to_lowercase()))
            .cloned()
            .collect())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, String> {
        if let Some(suggestion) = highlighted_suggestion {
            return Ok(Some(suggestion));
        }

        let suggestions = self.get_suggestions(input)?;
        if suggestions.is_empty() {
            return Ok(None);
        }

        if suggestions.len() == 1 {
            return Ok(Some(suggestions[0].clone()));
        }

        let first = &suggestions[0];
        let mut prefix = String::new();

        for (i, c) in first.chars().enumerate() {
            if suggestions
                .iter()
                .all(|s| s.chars().nth(i).map(|sc| sc == c).unwrap_or(false))
            {
                prefix.push(c);
            } else {
                break;
            }
        }

        if prefix.len() > input.len() {
            Ok(Some(prefix))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone)]
pub struct TextInput {
    prompt: String,
    default: Option<String>,
    placeholder: Option<String>,
    initial_value: Option<String>,
    inline: bool,
    prompt_prefix: String,
    help_message: Option<String>,
    show_suggestions: bool,
    suggestion_page_size: usize,
    allow_escape: bool,
    style: TextInputStyle,
    validation: Option<Box<dyn Validate<str>>>,
    autocomplete: Option<Box<dyn Autocomplete>>,
}

impl TextInput {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            default: None,
            placeholder: None,
            initial_value: None,
            inline: false,
            prompt_prefix: "?".into(),
            help_message: None,
            show_suggestions: true,
            suggestion_page_size: 5,
            allow_escape: true,
            style: TextInputStyle::default(),
            validation: None,
            autocomplete: None,
        }
    }

    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_initial_value(mut self, value: impl Into<String>) -> Self {
        self.initial_value = Some(value.into());
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

    pub fn with_suggestions(mut self, enabled: bool) -> Self {
        self.show_suggestions = enabled;
        self
    }

    pub fn with_suggestion_page_size(mut self, size: usize) -> Self {
        self.suggestion_page_size = size;
        self
    }

    pub fn with_escape(mut self, allow: bool) -> Self {
        self.allow_escape = allow;
        self
    }

    pub fn with_style(mut self, style: TextInputStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_validation(mut self, validation: impl Validate<str> + 'static) -> Self {
        self.validation = Some(Box::new(validation));
        self
    }

    pub fn with_autocomplete<A: Autocomplete + 'static>(mut self, autocomplete: A) -> Self {
        self.autocomplete = Some(Box::new(autocomplete));
        self
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn ask(&mut self) -> miette::Result<String> {
        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = terminal::disable_raw_mode();
            std::panic::take_hook()(panic_info);
        }));

        let result = self._ask_internal();

        let _ = std::panic::take_hook();
        std::panic::set_hook(original_hook);

        result
    }

    pub fn _ask_internal(&mut self) -> miette::Result<String> {
        let mut input = self.initial_value.clone().unwrap_or_default();
        let mut cursor_pos = input.len();
        let mut suggestions: Vec<String> = Vec::new();
        let mut selected_suggestion: Option<usize> = None;

        terminal::enable_raw_mode().into_diagnostic()?;

        while event::poll(std::time::Duration::from_millis(0)).into_diagnostic()? {
            event::read().into_diagnostic()?;
        }

        if let Some(ref mut ac) = self.autocomplete {
            suggestions = ac.get_suggestions(&input).unwrap_or_default();
        }

        let mut last_render_lines =
            self.render(&mut stdout(), &input, &suggestions, selected_suggestion)?;
        stdout().flush().into_diagnostic()?;

        loop {
            if let Event::Key(key_event) = event::read().into_diagnostic()? {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                match self.handle_key(
                    key_event,
                    &mut input,
                    &mut cursor_pos,
                    &mut suggestions,
                    &mut selected_suggestion,
                    &mut stdout(),
                ) {
                    Ok(Some(answer)) => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        if last_render_lines > 0 {
                            execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                                .into_diagnostic()?;
                        }
                        execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                        execute!(stdout(), Clear(ClearType::FromCursorDown)).into_diagnostic()?;
                        self.show_result(&mut stdout(), &answer)?;
                        return Ok(answer);
                    }
                    Ok(None) => {
                        if last_render_lines > 0 {
                            execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                                .into_diagnostic()?;
                        }
                        execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                        execute!(stdout(), Clear(ClearType::FromCursorDown)).into_diagnostic()?;
                        last_render_lines =
                            self.render(&mut stdout(), &input, &suggestions, selected_suggestion)?;
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
        }
    }

    fn handle_key(
        &mut self,
        key_event: KeyEvent,
        input: &mut String,
        cursor_pos: &mut usize,
        suggestions: &mut Vec<String>,
        selected_suggestion: &mut Option<usize>,
        _out: &mut std::io::Stdout,
    ) -> Result<Option<String>, String> {
        if key_event.modifiers.contains(KeyModifiers::CONTROL)
            && matches!(key_event.code, KeyCode::Char('c'))
        {
            let _ = terminal::disable_raw_mode();
            std::process::exit(130);
        }

        match key_event.code {
            KeyCode::Enter => {
                if let Some(idx) = *selected_suggestion
                    && let Some(suggestion) = suggestions.get(idx)
                {
                    *input = suggestion.clone();
                    *cursor_pos = input.len();
                }

                let final_input = if input.is_empty() {
                    self.default.clone().unwrap_or_default()
                } else {
                    input.clone()
                };

                self.validate_and_return(&final_input)
            }
            KeyCode::Char(c) => {
                input.insert(*cursor_pos, c);
                *cursor_pos += 1;
                *selected_suggestion = None;

                if let Some(ref mut ac) = self.autocomplete {
                    *suggestions = ac.get_suggestions(input).unwrap_or_default();
                }

                Ok(None)
            }
            KeyCode::Backspace if *cursor_pos > 0 => {
                *cursor_pos -= 1;
                input.remove(*cursor_pos);
                *selected_suggestion = None;

                if let Some(ref mut ac) = self.autocomplete {
                    *suggestions = ac.get_suggestions(input).unwrap_or_default();
                }

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
            KeyCode::Up if !suggestions.is_empty() => {
                *selected_suggestion = Some(match *selected_suggestion {
                    None => suggestions.len() - 1,
                    Some(0) => suggestions.len() - 1,
                    Some(n) => n - 1,
                });
                Ok(None)
            }
            KeyCode::Down if !suggestions.is_empty() => {
                *selected_suggestion = Some(match *selected_suggestion {
                    None => 0,
                    Some(n) if n >= suggestions.len() - 1 => 0,
                    Some(n) => n + 1,
                });
                Ok(None)
            }
            KeyCode::Tab if self.autocomplete.is_some() => {
                if let Some(ref mut ac) = self.autocomplete {
                    let highlighted =
                        selected_suggestion.and_then(|idx| suggestions.get(idx).cloned());

                    if let Ok(Some(replacement)) = ac.get_completion(input, highlighted) {
                        *input = replacement;
                        *cursor_pos = input.len();
                        *selected_suggestion = None;

                        *suggestions = ac.get_suggestions(input).unwrap_or_default();
                    }
                }
                Ok(None)
            }
            KeyCode::Esc if self.allow_escape => Err("Cancelled".into()),
            _ => Ok(None),
        }
    }

    pub fn validate_and_return(&self, value: &str) -> Result<Option<String>, String> {
        if let Some(ref validator) = self.validation {
            run_validator(validator.as_ref(), value)?;
        }

        Ok(Some(value.to_string()))
    }

    pub fn render(
        &self,
        out: &mut std::io::Stdout,
        input: &str,
        suggestions: &[String],
        selected_suggestion: Option<usize>,
    ) -> miette::Result<usize> {
        let tw = crate::util::term_width();
        let mut line_count = 0;

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
            line_count += crate::util::writeln_physical(out, &line, tw)?;
        }

        if let Some(ref help) = self.help_message {
            let line = format!("  {}", help.style(self.style.hint));
            line_count += crate::util::writeln_physical(out, &line, tw)?;
        }

        let display_text = if input.is_empty() {
            self.placeholder
                .as_deref()
                .unwrap_or("")
                .style(self.style.placeholder)
                .to_string()
        } else {
            input.style(self.style.input).to_string()
        };

        let mut input_line = format!("  {} ", display_text);

        if input.is_empty()
            && let Some(default) = self.default.clone()
        {
            input_line = format!(
                "{}(default: {}) ",
                input_line,
                &default.style(self.style.default_value)
            );
        }

        line_count += crate::util::writeln_physical(out, &input_line, tw)?;

        if self.show_suggestions && !suggestions.is_empty() {
            let visible_suggestions: Vec<_> = suggestions
                .iter()
                .enumerate()
                .take(self.suggestion_page_size)
                .collect();

            if !visible_suggestions.is_empty() {
                let line = format!("  {}", "Suggestions:".style(self.style.hint));
                line_count += crate::util::writeln_physical(out, &line, tw)?;

                for (idx, suggestion) in visible_suggestions {
                    let marker = if Some(idx) == selected_suggestion {
                        "▸"
                    } else {
                        " "
                    };

                    let style = if Some(idx) == selected_suggestion {
                        self.style.selected
                    } else {
                        self.style.suggestion
                    };

                    let line = format!(
                        "    {} {}",
                        marker.style(self.style.selected),
                        suggestion.style(style)
                    );
                    line_count += crate::util::writeln_physical(out, &line, tw)?;
                }

                if suggestions.len() > self.suggestion_page_size {
                    let line = format!(
                        "    {}",
                        format!(
                            "({} more...)",
                            suggestions.len() - self.suggestion_page_size
                        )
                        .style(self.style.hint)
                    );
                    line_count += crate::util::writeln_physical(out, &line, tw)?;
                }
            }
        }

        let mut hints = vec![];
        if self.autocomplete.is_some() {
            hints.push("Tab to autocomplete");
        }
        if !suggestions.is_empty() {
            hints.push("↑↓ to navigate");
        }
        hints.push("Enter to submit");
        if self.allow_escape {
            hints.push("Esc to cancel");
        }

        if !hints.is_empty() {
            let line = format!("  {}", hints.join(", ").style(self.style.hint));
            line_count += crate::util::writeln_physical(out, &line, tw)?;
        }

        Ok(line_count)
    }

    pub fn show_error(&self, out: &mut std::io::Stdout, error: &str) -> miette::Result<()> {
        let tw = crate::util::term_width();
        let line = format!(
            "{} {}",
            "✗".style(self.style.error),
            error.style(self.style.error_hint),
        );
        crate::util::writeln_physical(out, &line, tw)?;

        Ok(())
    }

    pub fn show_result(&self, out: &mut std::io::Stdout, answer: &str) -> miette::Result<()> {
        let tw = crate::util::term_width();
        let line = format!(
            "{} {} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            self.prompt.style(self.style.prompt),
            answer.style(self.style.input).bold(),
        );
        crate::util::writeln_physical(out, &line, tw)?;

        out.flush().into_diagnostic()?;
        Ok(())
    }
}
