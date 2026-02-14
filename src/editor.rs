use {
    crate::{
        style::EditorStyle,
        validation::{Validate, run_validator},
    },
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
        terminal,
    },
    miette::IntoDiagnostic,
    owo_colors::OwoColorize,
    std::{
        env,
        io::{Write, stdout},
        process::Command,
    },
};

#[derive(Clone)]
pub struct Editor {
    prompt: String,
    prompt_prefix: String,
    help_message: Option<String>,
    editor_command: Option<String>,
    file_extension: String,
    predefined_message: Option<String>,
    require_changes: bool,
    show_hints: bool,
    allow_escape: bool,
    style: EditorStyle,
    validation: Option<Box<dyn Validate<str>>>,
}

impl Editor {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            prompt_prefix: "📝".into(),
            help_message: None,
            editor_command: None,
            file_extension: "txt".into(),
            predefined_message: None,
            require_changes: false,
            show_hints: true,
            allow_escape: true,
            style: EditorStyle::default(),
            validation: None,
        }
    }

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

    pub fn with_editor(mut self, command: impl Into<String>) -> Self {
        self.editor_command = Some(command.into());
        self
    }

    pub fn with_file_extension(mut self, ext: impl Into<String>) -> Self {
        self.file_extension = ext.into();
        self
    }

    pub fn with_predefined_message(mut self, message: impl Into<String>) -> Self {
        self.predefined_message = Some(message.into());
        self
    }

    pub fn with_require_changes(mut self, require: bool) -> Self {
        self.require_changes = require;
        self
    }

    pub fn with_hints(mut self, enabled: bool) -> Self {
        self.show_hints = enabled;
        self
    }

    pub fn with_escape(mut self, allow: bool) -> Self {
        self.allow_escape = allow;
        self
    }

    pub fn with_style(mut self, style: EditorStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_validation(mut self, validation: impl Validate<str> + 'static) -> Self {
        self.validation = Some(Box::new(validation));
        self
    }

    fn detect_editor(&self) -> String {
        if let Some(ref cmd) = self.editor_command {
            return cmd.clone();
        }

        if let Ok(editor) = env::var("VISUAL") {
            return editor;
        }
        if let Ok(editor) = env::var("EDITOR") {
            return editor;
        }

        if cfg!(windows) {
            "notepad".into()
        } else {
            "nano".into()
        }
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
        let mut out = stdout();
        let editor = self.detect_editor();
        let tw = crate::util::term_width();

        let line = format!(
            "{} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            self.prompt.style(self.style.prompt),
        );
        crate::util::writeln_physical(&mut out, &line, tw)?;

        if let Some(ref help) = self.help_message {
            let line = format!("  {}", help.style(self.style.hint));
            crate::util::writeln_physical(&mut out, &line, tw)?;
        }

        let line = format!(
            "  {} {}",
            "Editor:".style(self.style.hint),
            editor.style(self.style.editor_command),
        );
        crate::util::writeln_physical(&mut out, &line, tw)?;

        if self.show_hints {
            let mut hints = vec!["Enter to open editor"];
            if self.allow_escape {
                hints.push("Esc to cancel");
            }
            let line = format!("  {}", hints.join(", ").style(self.style.hint));
            crate::util::writeln_physical(&mut out, &line, tw)?;
        }

        out.flush().into_diagnostic()?;

        terminal::enable_raw_mode().into_diagnostic()?;

        while event::poll(std::time::Duration::from_millis(0)).into_diagnostic()? {
            event::read().into_diagnostic()?;
        }

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

                match key_event.code {
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        break;
                    }
                    KeyCode::Esc if self.allow_escape => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        return Err(miette::miette!("Cancelled"));
                    }
                    _ => continue,
                }
            }
        }

        let temp_dir = env::temp_dir();
        let file_name = format!("bearask_{}.{}", std::process::id(), self.file_extension);
        let temp_path = temp_dir.join(file_name);

        if let Some(ref message) = self.predefined_message {
            std::fs::write(&temp_path, message).into_diagnostic()?;
        } else {
            std::fs::write(&temp_path, "").into_diagnostic()?;
        }

        let editor_parts: Vec<&str> = editor.split_whitespace().collect();
        let (program, args) = editor_parts
            .split_first()
            .ok_or_else(|| miette::miette!("Empty editor command"))?;

        let status = Command::new(program)
            .args(args)
            .arg(&temp_path)
            .status()
            .into_diagnostic()?;

        if !status.success() {
            let _ = std::fs::remove_file(&temp_path);
            return Err(miette::miette!("Editor exited with status: {}", status));
        }

        let content = std::fs::read_to_string(&temp_path).into_diagnostic()?;
        let _ = std::fs::remove_file(&temp_path);

        let trimmed = content.trim().to_string();

        if self.require_changes {
            let original = self.predefined_message.as_deref().unwrap_or("").trim();
            if trimmed == original {
                return Err(miette::miette!("No changes were made"));
            }
        }

        if let Some(ref validator) = self.validation {
            run_validator(validator.as_ref(), &trimmed).map_err(|e| miette::miette!(e))?;
        }

        let line_count = trimmed.lines().count();
        let line = format!(
            "{} {} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            self.prompt.style(self.style.prompt),
            format!(
                "({} line{})",
                line_count,
                if line_count == 1 { "" } else { "s" }
            )
            .style(self.style.success)
            .bold(),
        );
        crate::util::writeln_physical(&mut out, &line, tw)?;
        out.flush().into_diagnostic()?;

        Ok(trimmed)
    }
}
