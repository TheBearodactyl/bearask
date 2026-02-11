use {
    crate::{
        style::NumberStyle,
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
    std::{
        fmt::Display,
        io::{Write, stdout},
        str::FromStr,
    },
};

pub trait NumericType: FromStr + Display + PartialOrd + Copy + 'static {
    fn increment(self, step: Self) -> Self;
    fn decrement(self, step: Self) -> Self;
}

macro_rules! impl_numeric_type {
    ($($t:ty),*) => {
        $(
            impl NumericType for $t {
                fn increment(self, step: Self) -> Self { self + step }
                fn decrement(self, step: Self) -> Self { self - step }
            }
        )*
    };
}

impl_numeric_type!(
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, isize, usize
);

#[derive(Clone)]
pub struct Number<T: NumericType> {
    prompt: String,
    prompt_prefix: String,
    help_message: Option<String>,
    default: Option<T>,
    min: Option<T>,
    max: Option<T>,
    step: T,
    allow_escape: bool,
    show_hints: bool,
    show_bounds: bool,
    style: NumberStyle,
    validation: Option<Box<dyn Validate<T>>>,
}

impl<T: NumericType> Number<T>
where
    T: From<u8>,
{
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            prompt_prefix: "#".into(),
            help_message: None,
            default: None,
            min: None,
            max: None,
            step: T::from(1),
            allow_escape: true,
            show_hints: true,
            show_bounds: true,
            style: NumberStyle::default(),
            validation: None,
        }
    }
}

impl<T: NumericType> Number<T> {
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn with_prompt_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prompt_prefix = prefix.into();
        self
    }

    pub fn with_help_message(mut self, message: impl Into<String>) -> Self {
        self.help_message = Some(message.into());
        self
    }

    pub fn with_default(mut self, default: T) -> Self {
        self.default = Some(default);
        self
    }

    pub fn with_min(mut self, min: T) -> Self {
        self.min = Some(min);
        self
    }

    pub fn with_max(mut self, max: T) -> Self {
        self.max = Some(max);
        self
    }

    pub fn with_step(mut self, step: T) -> Self {
        self.step = step;
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

    pub fn with_bounds(mut self, show: bool) -> Self {
        self.show_bounds = show;
        self
    }

    pub fn with_style(mut self, style: NumberStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_validation(mut self, validation: impl Validate<T> + 'static) -> Self {
        self.validation = Some(Box::new(validation));
        self
    }

    pub fn ask(&self) -> miette::Result<T> {
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

    fn ask_internal(&self) -> miette::Result<T> {
        let mut input = self.default.map(|d| d.to_string()).unwrap_or_default();
        let mut cursor_pos = input.len();
        let mut error_message: Option<String> = None;

        terminal::enable_raw_mode().into_diagnostic()?;

        while event::poll(std::time::Duration::from_millis(0)).into_diagnostic()? {
            event::read().into_diagnostic()?;
        }

        let mut last_render_lines = self.render(&mut stdout(), &input, error_message.as_deref())?;
        stdout().flush().into_diagnostic()?;

        loop {
            if let Event::Key(key_event) = event::read().into_diagnostic()? {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                error_message = None;

                match self.handle_key(key_event, &mut input, &mut cursor_pos) {
                    Ok(Some(value)) => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        if last_render_lines > 0 {
                            execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                                .into_diagnostic()?;
                        }
                        execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                        execute!(stdout(), Clear(ClearType::FromCursorDown)).into_diagnostic()?;
                        self.show_result(&mut stdout(), value)?;
                        return Ok(value);
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
                last_render_lines = self.render(&mut stdout(), &input, error_message.as_deref())?;
                stdout().flush().into_diagnostic()?;
            }
        }
    }

    fn handle_key(
        &self,
        key_event: KeyEvent,
        input: &mut String,
        cursor_pos: &mut usize,
    ) -> Result<Option<T>, String> {
        if key_event.modifiers.contains(KeyModifiers::CONTROL)
            && matches!(key_event.code, KeyCode::Char('c'))
        {
            let _ = terminal::disable_raw_mode();
            std::process::exit(130);
        }

        match key_event.code {
            KeyCode::Enter => {
                if input.is_empty() {
                    if let Some(default) = self.default {
                        return self.validate_value(default);
                    }
                    return Err("Please enter a number".to_string());
                }

                let value: T = input
                    .parse()
                    .map_err(|_| format!("Invalid number: {}", input))?;
                self.validate_value(value)
            }
            KeyCode::Up => {
                if let Ok(current) = input.parse::<T>() {
                    let new_val = current.increment(self.step);
                    if let Some(max) = self.max {
                        if new_val > max {
                            return Ok(None);
                        }
                    }
                    *input = new_val.to_string();
                    *cursor_pos = input.len();
                } else if let Some(default) = self.default {
                    *input = default.to_string();
                    *cursor_pos = input.len();
                }
                Ok(None)
            }
            KeyCode::Down => {
                if let Ok(current) = input.parse::<T>() {
                    let new_val = current.decrement(self.step);
                    if let Some(min) = self.min {
                        if new_val < min {
                            return Ok(None);
                        }
                    }
                    *input = new_val.to_string();
                    *cursor_pos = input.len();
                } else if let Some(default) = self.default {
                    *input = default.to_string();
                    *cursor_pos = input.len();
                }
                Ok(None)
            }
            KeyCode::Char(c) if c.is_ascii_digit() || c == '-' || c == '.' => {
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

    fn validate_value(&self, value: T) -> Result<Option<T>, String> {
        if let Some(min) = self.min {
            if value < min {
                return Err(format!("Value must be at least {}", min));
            }
        }
        if let Some(max) = self.max {
            if value > max {
                return Err(format!("Value must be at most {}", max));
            }
        }
        if let Some(ref validator) = self.validation {
            run_validator(validator.as_ref(), &value)?;
        }
        Ok(Some(value))
    }

    fn render(
        &self,
        out: &mut std::io::Stdout,
        input: &str,
        error: Option<&str>,
    ) -> miette::Result<usize> {
        let mut line_count = 0;

        write!(
            out,
            "{} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            self.prompt.style(self.style.prompt),
        )
        .into_diagnostic()?;

        if self.show_bounds && (self.min.is_some() || self.max.is_some()) {
            let min_str = self
                .min
                .map(|m| m.to_string())
                .unwrap_or_else(|| "-∞".to_string());
            let max_str = self
                .max
                .map(|m| m.to_string())
                .unwrap_or_else(|| "∞".to_string());
            write!(
                out,
                " {}",
                format!("[{}..{}]", min_str, max_str).style(self.style.bounds)
            )
            .into_diagnostic()?;
        }

        writeln!(out).into_diagnostic()?;
        line_count += 1;

        if let Some(ref help) = self.help_message {
            writeln!(out, "  {}", help.style(self.style.hint)).into_diagnostic()?;
            line_count += 1;
        }

        let display_text = if input.is_empty() {
            if let Some(default) = self.default {
                format!("(default: {})", default)
                    .style(self.style.default_value)
                    .to_string()
            } else {
                " ".to_string()
            }
        } else {
            input.style(self.style.input).to_string()
        };

        writeln!(out, "  {}", display_text).into_diagnostic()?;
        line_count += 1;

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
            let mut hints = vec!["↑↓ to increment/decrement"];
            hints.push("Enter to submit");
            if self.allow_escape {
                hints.push("Esc to cancel");
            }

            writeln!(out, "  {}", hints.join(", ").style(self.style.hint)).into_diagnostic()?;
            line_count += 1;
        }

        Ok(line_count)
    }

    fn show_result(&self, out: &mut std::io::Stdout, value: T) -> miette::Result<()> {
        writeln!(
            out,
            "{} {} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            self.prompt.style(self.style.prompt),
            value.to_string().style(self.style.input).bold(),
        )
        .into_diagnostic()?;

        out.flush().into_diagnostic()?;
        Ok(())
    }
}
