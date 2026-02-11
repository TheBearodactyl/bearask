use {
    crate::{
        option::AskOption,
        style::MultiSelectStyle,
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
        collections::HashSet,
        io::{Write, stdout},
    },
};

#[derive(Clone)]
#[allow(clippy::type_complexity)]
pub struct MultiSelect<T: Clone> {
    prompt: String,
    options: Vec<AskOption<T>>,
    default_selections: HashSet<usize>,
    page_size: usize,
    prompt_prefix: String,
    help_message: Option<String>,
    show_hints: bool,
    show_descriptions: bool,
    allow_escape: bool,
    vim_mode: bool,
    min_selections: Option<usize>,
    max_selections: Option<usize>,
    style: MultiSelectStyle,
    validation: Option<Box<dyn Validate<[usize]>>>,
}

impl<T: Clone> MultiSelect<T> {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            options: Vec::new(),
            default_selections: HashSet::new(),
            page_size: 10,
            prompt_prefix: "?".into(),
            help_message: None,
            show_hints: true,
            show_descriptions: true,
            allow_escape: true,
            vim_mode: false,
            min_selections: None,
            max_selections: None,
            style: MultiSelectStyle::default(),
            validation: None,
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

    pub fn with_default_selections(mut self, indices: &[usize]) -> Self {
        self.default_selections = indices.iter().copied().collect();
        self
    }

    pub fn with_all_selected(mut self) -> Self {
        self.default_selections = (0..self.options.len()).collect();
        self
    }

    pub fn with_page_size(mut self, size: usize) -> Self {
        self.page_size = size.max(3);
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

    pub fn with_min_selections(mut self, min: usize) -> Self {
        self.min_selections = Some(min);
        self
    }

    pub fn with_max_selections(mut self, max: usize) -> Self {
        self.max_selections = Some(max);
        self
    }

    pub fn with_style(mut self, style: MultiSelectStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_validation(mut self, validation: impl Validate<[usize]> + 'static) -> Self {
        self.validation = Some(Box::new(validation));
        self
    }

    pub fn ask(&self) -> miette::Result<Vec<usize>> {
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

    fn ask_internal(&self) -> miette::Result<Vec<usize>> {
        let mut selected_indices: HashSet<usize> = self
            .default_selections
            .iter()
            .copied()
            .filter(|&i| i < self.options.len())
            .collect();

        let mut cursor = 0;
        let mut scroll_offset = 0;
        let mut last_render_lines =
            self.render(&mut stdout(), cursor, scroll_offset, &selected_indices)?;

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

                match self.handle_key(
                    key_event,
                    &mut cursor,
                    &mut scroll_offset,
                    &mut selected_indices,
                ) {
                    Ok(Some(())) => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        if last_render_lines > 0 {
                            execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                                .into_diagnostic()?;
                        }
                        execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                        execute!(stdout(), Clear(ClearType::FromCursorDown)).into_diagnostic()?;

                        let mut result: Vec<usize> = selected_indices.into_iter().collect();
                        result.sort_unstable();
                        self.show_result(&mut stdout(), &result)?;
                        return Ok(result);
                    }
                    Ok(None) => {
                        if last_render_lines > 0 {
                            execute!(stdout(), cursor::MoveUp(last_render_lines as u16))
                                .into_diagnostic()?;
                        }
                        execute!(stdout(), cursor::MoveToColumn(0)).into_diagnostic()?;
                        execute!(stdout(), Clear(ClearType::FromCursorDown)).into_diagnostic()?;
                        last_render_lines =
                            self.render(&mut stdout(), cursor, scroll_offset, &selected_indices)?;
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
        cursor: &mut usize,
        scroll_offset: &mut usize,
        selected_indices: &mut HashSet<usize>,
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
                if *cursor > 0 {
                    *cursor -= 1;
                    if *cursor < *scroll_offset {
                        *scroll_offset = *cursor;
                    }
                } else {
                    *cursor = total_options - 1;
                    *scroll_offset = cursor.saturating_sub(self.page_size - 1);
                }
                Ok(None)
            }
            KeyCode::Down | KeyCode::Char('j')
                if self.vim_mode || key_event.code == KeyCode::Down =>
            {
                if *cursor < total_options - 1 {
                    *cursor += 1;
                    if *cursor >= *scroll_offset + self.page_size {
                        *scroll_offset = cursor.saturating_sub(self.page_size - 1);
                    }
                } else {
                    *cursor = 0;
                    *scroll_offset = 0;
                }
                Ok(None)
            }
            KeyCode::Home | KeyCode::Char('g')
                if self.vim_mode || key_event.code == KeyCode::Home =>
            {
                *cursor = 0;
                *scroll_offset = 0;
                Ok(None)
            }
            KeyCode::End | KeyCode::Char('G')
                if self.vim_mode || key_event.code == KeyCode::End =>
            {
                *cursor = total_options - 1;
                *scroll_offset = cursor.saturating_sub(self.page_size - 1);
                Ok(None)
            }
            KeyCode::PageUp => {
                *cursor = cursor.saturating_sub(self.page_size);
                *scroll_offset = scroll_offset.saturating_sub(self.page_size);
                Ok(None)
            }
            KeyCode::PageDown => {
                *cursor = (*cursor + self.page_size).min(total_options - 1);
                if *cursor >= *scroll_offset + self.page_size {
                    *scroll_offset = cursor.saturating_sub(self.page_size - 1);
                }
                Ok(None)
            }
            KeyCode::Char(' ') => {
                if selected_indices.contains(cursor) {
                    selected_indices.remove(cursor);
                } else {
                    if let Some(max) = self.max_selections
                        && selected_indices.len() >= max
                    {
                        return Ok(None);
                    }
                    selected_indices.insert(*cursor);
                }
                Ok(None)
            }
            KeyCode::Right | KeyCode::Char('a')
                if self.vim_mode || key_event.code == KeyCode::Right =>
            {
                if let Some(max) = self.max_selections
                    && total_options > max
                {
                    return Err(format!(
                        "Cannot select all: maximum {} selections allowed",
                        max
                    ));
                }
                *selected_indices = (0..total_options).collect();
                Ok(None)
            }
            KeyCode::Left | KeyCode::Char('d')
                if self.vim_mode || key_event.code == KeyCode::Left =>
            {
                selected_indices.clear();
                Ok(None)
            }
            KeyCode::Char('i') if self.vim_mode => {
                let all_indices: HashSet<usize> = (0..total_options).collect();
                let new_selections: HashSet<usize> =
                    all_indices.difference(selected_indices).copied().collect();

                if let Some(max) = self.max_selections
                    && new_selections.len() > max
                {
                    return Err(format!(
                        "Cannot invert: would exceed maximum {} selections",
                        max
                    ));
                }
                *selected_indices = new_selections;
                Ok(None)
            }
            KeyCode::Enter => {
                let selected_vec: Vec<usize> = selected_indices.iter().copied().collect();
                self.validate_and_return(&selected_vec)
            }
            KeyCode::Esc if self.allow_escape => Err("Cancelled".into()),
            _ => Ok(None),
        }
    }

    fn validate_and_return(&self, selected: &[usize]) -> Result<Option<()>, String> {
        if let Some(min) = self.min_selections
            && selected.len() < min
        {
            return Err(format!("Please select at least {} option(s)", min));
        }

        if let Some(max) = self.max_selections
            && selected.len() > max
        {
            return Err(format!("Please select at most {} option(s)", max));
        }

        if let Some(ref validator) = self.validation {
            run_validator(validator.as_ref(), selected)?;
        }

        Ok(Some(()))
    }

    fn render(
        &self,
        out: &mut std::io::Stdout,
        cursor: usize,
        scroll_offset: usize,
        selected_indices: &HashSet<usize>,
    ) -> miette::Result<usize> {
        let mut line_count = 0;

        writeln!(
            out,
            "{} {} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            self.prompt.style(self.style.prompt),
            format!(
                "(min: {}, max: {})",
                self.min_selections.unwrap_or(0),
                self.max_selections.unwrap_or(usize::MAX)
            )
            .style(self.style.hint)
        )
        .into_diagnostic()?;
        line_count += 1;

        if let Some(ref help) = self.help_message {
            writeln!(out, "  {}", help.style(self.style.hint)).into_diagnostic()?;
            line_count += 1;
        }

        let end_offset = (scroll_offset + self.page_size).min(self.options.len());
        let visible_options = &self.options[scroll_offset..end_offset];

        for (i, option) in visible_options.iter().enumerate() {
            let absolute_index = scroll_offset + i;
            let is_cursor = absolute_index == cursor;
            let is_selected = selected_indices.contains(&absolute_index);

            let cursor_marker = if is_cursor { "󰁕" } else { " " };
            let checkbox = if is_selected { "󰄲" } else { "󰄮" };

            if self.show_descriptions && !option.description.is_empty() {
                writeln!(
                    out,
                    "  {} {} {}",
                    cursor_marker.style(self.style.cursor),
                    checkbox.style(if is_selected {
                        self.style.checkbox_selected
                    } else {
                        self.style.checkbox_unselected
                    }),
                    option.name.style(option.name_style)
                )
                .into_diagnostic()?;
                line_count += 1;

                writeln!(
                    out,
                    "        {}",
                    option.description.style(option.description_style)
                )
                .into_diagnostic()?;
                line_count += 1;
            } else {
                let style = if is_cursor {
                    self.style.cursor
                } else if is_selected {
                    self.style.selected
                } else {
                    self.style.option_name
                };

                writeln!(
                    out,
                    "  {} {} {}",
                    cursor_marker.style(self.style.cursor),
                    checkbox.style(if is_selected {
                        self.style.checkbox_selected
                    } else {
                        self.style.checkbox_unselected
                    }),
                    option.name.style(style)
                )
                .into_diagnostic()?;
                line_count += 1;
            }
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

        if end_offset < self.options.len() {
            writeln!(
                out,
                "  {}",
                format!("(↓ {} more below)", self.options.len() - end_offset)
                    .style(self.style.hint)
            )
            .into_diagnostic()?;
            line_count += 1;
        }

        if self.show_hints {
            let mut hints = vec![];

            hints.push("Space to toggle");

            if self.vim_mode {
                hints.push("j/k or ↑/↓ to navigate");
                hints.push("a to select all");
                hints.push("d to deselect all");
                hints.push("i to invert");
            } else {
                hints.push("↑↓ to navigate");
                hints.push("→ to select all");
                hints.push("← to deselect all");
            }

            if self.options.len() > self.page_size {
                hints.push("PgUp/PgDn to scroll");
            }

            hints.push("Enter to submit");

            if self.allow_escape {
                hints.push("Esc to cancel");
            }

            writeln!(out, "  {}", hints.join(", ").style(self.style.hint)).into_diagnostic()?;
            line_count += 1;
        }

        let count_text = format!("[{} selected]", selected_indices.len());
        writeln!(out, "  {}", count_text.style(self.style.selection_count)).into_diagnostic()?;
        line_count += 1;

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

    fn show_result(&self, out: &mut std::io::Stdout, selected: &[usize]) -> miette::Result<()> {
        let selected_names: Vec<String> = selected
            .iter()
            .filter_map(|&i| self.options.get(i).map(|opt| opt.name.clone()))
            .collect();

        let result_text = if selected_names.is_empty() {
            "None".to_string()
        } else {
            selected_names.join(", ")
        };

        writeln!(
            out,
            "{} {} {}",
            self.prompt_prefix.style(self.style.prompt_prefix),
            self.prompt.style(self.style.prompt),
            result_text.style(self.style.selected).bold(),
        )
        .into_diagnostic()?;

        out.flush().into_diagnostic()?;
        Ok(())
    }
}
