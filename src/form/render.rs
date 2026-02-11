use {
    crate::form::{FieldKind, Form, InternalField},
    crossterm::{
        cursor, execute,
        terminal::{Clear, ClearType},
    },
    miette::IntoDiagnostic,
    owo_colors::OwoColorize,
    std::io::Write,
};

impl Form {
    pub fn clear_render(out: &mut std::io::Stdout, lines: usize) -> miette::Result<()> {
        if lines > 0 {
            execute!(out, cursor::MoveUp(lines as u16)).into_diagnostic()?;
        }

        execute!(out, cursor::MoveToColumn(0)).into_diagnostic()?;
        execute!(out, Clear(ClearType::FromCursorDown)).into_diagnostic()?;
        Ok(())
    }

    pub fn render(&self, out: &mut std::io::Stdout, active: usize) -> miette::Result<usize> {
        let mut lines = 0;

        if self.show_title {
            writeln!(
                out,
                "{} {}",
                self.style
                    .title_prefix
                    .as_deref()
                    .unwrap_or("◆")
                    .style(self.style.title_prefix_style),
                self.title.style(self.style.title),
            )
            .into_diagnostic()?;
            lines += 1;
        }

        let mut last_section: Option<usize> = None;

        for (idx, field) in self.fields.iter().enumerate() {
            if field.section_idx != last_section {
                if let Some(sec_idx) = field.section_idx
                    && let Some(sec) = self.sections.get(sec_idx)
                {
                    writeln!(out).into_diagnostic()?;
                    lines += 1;
                    writeln!(
                        out,
                        "  {} {}",
                        "─".style(self.style.section_decoration),
                        sec.title.style(self.style.section_title),
                    )
                    .into_diagnostic()?;
                    lines += 1;
                }
                last_section = field.section_idx;
            }

            let is_active = idx == active;

            if is_active {
                lines += self.render_field_active(out, field)?;
            } else if field.confirmed {
                lines += self.render_field_completed(out, field)?;
            } else {
                lines += self.render_field_pending(out, field)?;
            }
        }

        writeln!(out).into_diagnostic()?;
        lines += 1;

        let confirmed_count = self.fields.iter().filter(|f| f.confirmed).count();
        let progress = format!("[{}/{}]", confirmed_count, self.fields.len());

        writeln!(
            out,
            "  {} {}",
            "Tab/Shift-Tab: navigate, Enter: confirm, Esc: cancel".style(self.style.nav_hint),
            progress.style(self.style.progress),
        )
        .into_diagnostic()?;
        lines += 1;

        Ok(lines)
    }

    pub fn render_field_active(
        &self,
        out: &mut std::io::Stdout,
        field: &InternalField,
    ) -> miette::Result<usize> {
        let mut lines = 0;
        let style = &self.style;

        match &field.kind {
            FieldKind::Text {
                prompt,
                placeholder,
                default,
                help,
                input,
                ..
            } => {
                writeln!(
                    out,
                    "  {} {}",
                    "▸".style(style.active_marker),
                    prompt.style(style.active_prompt),
                )
                .into_diagnostic()?;
                lines += 1;

                if let Some(h) = help {
                    writeln!(out, "    {}", h.style(style.hint)).into_diagnostic()?;
                    lines += 1;
                }

                let display = if input.is_empty() {
                    if let Some(ph) = placeholder {
                        ph.style(style.placeholder).to_string()
                    } else if let Some(def) = default {
                        format!("(default: {})", def)
                            .style(style.default_value)
                            .to_string()
                    } else {
                        " ".into()
                    }
                } else {
                    input.style(style.active_input).to_string()
                };
                writeln!(out, "    {}", display).into_diagnostic()?;
                lines += 1;
            }

            FieldKind::Confirm {
                prompt,
                selected,
                yes_text,
                no_text,
                ..
            } => {
                writeln!(
                    out,
                    "  {} {}",
                    "▸".style(style.active_marker),
                    prompt.style(style.active_prompt),
                )
                .into_diagnostic()?;
                lines += 1;

                if *selected {
                    write!(
                        out,
                        "    {}  ",
                        format!("▸ {}", yes_text).style(style.active_selected)
                    )
                    .into_diagnostic()?;
                    writeln!(out, "{}", format!("  {}", no_text).style(style.option))
                        .into_diagnostic()?;
                } else {
                    write!(
                        out,
                        "    {}  ",
                        format!("  {}", yes_text).style(style.option)
                    )
                    .into_diagnostic()?;
                    writeln!(
                        out,
                        "{}",
                        format!("▸ {}", no_text).style(style.active_selected)
                    )
                    .into_diagnostic()?;
                }
                lines += 1;
            }

            FieldKind::Select {
                prompt,
                options,
                page_size,
                help,
                cursor,
                scroll_offset,
            } => {
                writeln!(
                    out,
                    "  {} {}",
                    "▸".style(style.active_marker),
                    prompt.style(style.active_prompt),
                )
                .into_diagnostic()?;
                lines += 1;

                if let Some(h) = help {
                    writeln!(out, "    {}", h.style(style.hint)).into_diagnostic()?;
                    lines += 1;
                }

                let end = (*scroll_offset + *page_size).min(options.len());

                for (i, opt) in options[*scroll_offset..end].iter().enumerate() {
                    let marker = if i + scroll_offset == *cursor {
                        "▸"
                    } else {
                        " "
                    };
                    let s = if i + scroll_offset == *cursor {
                        style.active_selected
                    } else {
                        style.option
                    };
                    writeln!(
                        out,
                        "    {} {}",
                        marker.style(style.active_selected),
                        opt.name.style(s),
                    )
                    .into_diagnostic()?;
                    lines += 1;
                }

                if *scroll_offset > 0 {
                    writeln!(
                        out,
                        "    {}",
                        format!("(↑ {} more)", scroll_offset).style(style.hint),
                    )
                    .into_diagnostic()?;
                    lines += 1;
                }
                if end < options.len() {
                    writeln!(
                        out,
                        "    {}",
                        format!("(↓ {} more)", options.len() - end).style(style.hint),
                    )
                    .into_diagnostic()?;
                    lines += 1;
                }
            }

            FieldKind::MultiSelect {
                prompt,
                options,
                page_size,
                help,
                cursor,
                scroll_offset,
                selected,
                ..
            } => {
                let sel_count = selected.len();
                writeln!(
                    out,
                    "  {} {} {}",
                    "▸".style(style.active_marker),
                    prompt.style(style.active_prompt),
                    format!("[{} selected]", sel_count).style(style.hint),
                )
                .into_diagnostic()?;
                lines += 1;

                if let Some(h) = help {
                    writeln!(out, "    {}", h.style(style.hint)).into_diagnostic()?;
                    lines += 1;
                }

                let end = (*scroll_offset + *page_size).min(options.len());
                for (i, opt) in options[*scroll_offset..end].iter().enumerate() {
                    let actual_index = *scroll_offset + i;
                    let is_cur = actual_index == *cursor;
                    let is_sel = selected.contains(&actual_index);
                    let marker = if is_cur { "▸" } else { " " };
                    let checkbox = if is_sel { "[✓]" } else { "[ ]" };
                    let s = if is_cur {
                        style.active_selected
                    } else if is_sel {
                        style.checkbox_on
                    } else {
                        style.option
                    };
                    writeln!(
                        out,
                        "    {} {} {}",
                        marker.style(style.active_selected),
                        checkbox.style(if is_sel {
                            style.checkbox_on
                        } else {
                            style.checkbox_off
                        }),
                        opt.name.style(s),
                    )
                    .into_diagnostic()?;
                    lines += 1;
                }

                if *scroll_offset > 0 {
                    writeln!(
                        out,
                        "    {}",
                        format!("(↑ {} more)", scroll_offset).style(style.hint),
                    )
                    .into_diagnostic()?;
                    lines += 1;
                }
                if end < options.len() {
                    writeln!(
                        out,
                        "    {}",
                        format!("(↓ {} more)", options.len() - end).style(style.hint),
                    )
                    .into_diagnostic()?;
                    lines += 1;
                }
            }

            FieldKind::Int {
                prompt,
                help,
                input,
                ..
            }
            | FieldKind::Float {
                prompt,
                help,
                input,
                default: _,
                min: _,
                max: _,
                ..
            } => {
                let bounds = match &field.kind {
                    FieldKind::Int { min, max, .. } => {
                        if min.is_some() || max.is_some() {
                            let lo = min
                                .map(|m| m.to_string())
                                .unwrap_or_else(|| "-∞".into());
                            let hi = max
                                .map(|m| m.to_string())
                                .unwrap_or_else(|| "∞".into());
                            Some(format!("[{}..{}]", lo, hi))
                        } else {
                            None
                        }
                    }
                    FieldKind::Float { min, max, .. } => {
                        if min.is_some() || max.is_some() {
                            let lo = min
                                .map(|m| m.to_string())
                                .unwrap_or_else(|| "-∞".into());
                            let hi = max
                                .map(|m| m.to_string())
                                .unwrap_or_else(|| "∞".into());
                            Some(format!("[{}..{}]", lo, hi))
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                write!(
                    out,
                    "  {} {}",
                    "▸".style(style.active_marker),
                    prompt.style(style.active_prompt),
                )
                .into_diagnostic()?;
                if let Some(b) = bounds {
                    write!(out, " {}", b.style(style.hint)).into_diagnostic()?;
                }
                writeln!(out).into_diagnostic()?;
                lines += 1;

                if let Some(h) = help {
                    writeln!(out, "    {}", h.style(style.hint)).into_diagnostic()?;
                    lines += 1;
                }

                let display = if input.is_empty() {
                    let def_str = match &field.kind {
                        FieldKind::Int { default, .. } => default.map(|d| d.to_string()),
                        FieldKind::Float { default, .. } => default.map(|d| d.to_string()),
                        _ => None,
                    };
                    if let Some(ds) = def_str {
                        format!("(default: {})", ds)
                            .style(style.default_value)
                            .to_string()
                    } else {
                        " ".into()
                    }
                } else {
                    input.style(style.active_input).to_string()
                };
                writeln!(out, "    {}", display).into_diagnostic()?;
                lines += 1;
            }

            FieldKind::Password {
                prompt,
                mask_char,
                help,
                input,
                revealed,
                ..
            } => {
                writeln!(
                    out,
                    "  {} {}",
                    "▸".style(style.active_marker),
                    prompt.style(style.active_prompt),
                )
                .into_diagnostic()?;
                lines += 1;

                if let Some(h) = help {
                    writeln!(out, "    {}", h.style(style.hint)).into_diagnostic()?;
                    lines += 1;
                }

                let display = if input.is_empty() {
                    " ".into()
                } else if *revealed {
                    input.style(style.active_input).to_string()
                } else {
                    mask_char
                        .to_string()
                        .repeat(input.len())
                        .style(style.password_mask)
                        .to_string()
                };
                writeln!(out, "    {}", display).into_diagnostic()?;
                lines += 1;
            }

            FieldKind::Sort {
                prompt,
                page_size,
                help,
                items,
                cursor,
                grabbed,
                scroll_offset,
            } => {
                writeln!(
                    out,
                    "  {} {}",
                    "▸".style(style.active_marker),
                    prompt.style(style.active_prompt),
                )
                .into_diagnostic()?;
                lines += 1;

                if let Some(h) = help {
                    writeln!(out, "    {}", h.style(style.hint)).into_diagnostic()?;
                    lines += 1;
                }

                let end = (*scroll_offset + *page_size).min(items.len());
                for (i, item) in items[*scroll_offset..end].iter().enumerate() {
                    let actual_index = *scroll_offset + i;
                    let is_cur = actual_index == *cursor;
                    let (marker, item_style) = if is_cur && *grabbed {
                        ("≡", style.sort_grabbed)
                    } else if is_cur {
                        ("▸", style.active_selected)
                    } else {
                        (" ", style.option)
                    };
                    writeln!(
                        out,
                        "    {} {} {}",
                        marker.style(if is_cur && *grabbed {
                            style.sort_grabbed
                        } else {
                            style.active_selected
                        }),
                        format!("{}.", actual_index + 1).style(style.hint),
                        item.style(item_style),
                    )
                    .into_diagnostic()?;
                    lines += 1;
                }

                if *scroll_offset > 0 {
                    writeln!(
                        out,
                        "    {}",
                        format!("(↑ {} more)", scroll_offset).style(style.hint),
                    )
                    .into_diagnostic()?;
                    lines += 1;
                }
                if end < items.len() {
                    writeln!(
                        out,
                        "    {}",
                        format!("(↓ {} more)", items.len() - end).style(style.hint),
                    )
                    .into_diagnostic()?;
                    lines += 1;
                }
            }
        }

        if let Some(ref err) = field.error {
            writeln!(
                out,
                "    {} {}",
                "✗".style(style.error),
                err.style(style.error_hint),
            )
            .into_diagnostic()?;
            lines += 1;
        }

        Ok(lines)
    }

    pub fn render_field_completed(
        &self,
        out: &mut std::io::Stdout,
        field: &InternalField,
    ) -> miette::Result<usize> {
        let summary = field.kind.value_summary();
        writeln!(
            out,
            "  {} {} {}",
            "✓".style(self.style.completed_prefix),
            field.kind.prompt_text().style(self.style.prompt),
            summary.style(self.style.completed_value).bold(),
        )
        .into_diagnostic()?;
        Ok(1)
    }

    pub fn render_field_pending(
        &self,
        out: &mut std::io::Stdout,
        field: &InternalField,
    ) -> miette::Result<usize> {
        writeln!(
            out,
            "  {} {}",
            "·".style(self.style.pending_prefix),
            field.kind.prompt_text().style(self.style.pending_prompt),
        )
        .into_diagnostic()?;
        Ok(1)
    }

    pub fn render_final(&self, out: &mut std::io::Stdout) -> miette::Result<()> {
        if self.show_title {
            writeln!(
                out,
                "{} {}",
                self.style
                    .title_prefix
                    .as_deref()
                    .unwrap_or("◆")
                    .style(self.style.title_prefix_style),
                self.title.style(self.style.title),
            )
            .into_diagnostic()?;
        }

        let mut last_section: Option<usize> = None;
        for field in &self.fields {
            if field.section_idx != last_section {
                if let Some(sec_idx) = field.section_idx
                    && let Some(sec) = self.sections.get(sec_idx)
                {
                    writeln!(out).into_diagnostic()?;
                    writeln!(
                        out,
                        "  {} {}",
                        "─".style(self.style.section_decoration),
                        sec.title.style(self.style.section_title),
                    )
                    .into_diagnostic()?;
                }
                last_section = field.section_idx;
            }
            self.render_field_completed(out, field)?;
        }

        if self.show_summary {
            writeln!(out).into_diagnostic()?;
            writeln!(
                out,
                "{} {}",
                self.style
                    .summary_prefix
                    .as_deref()
                    .unwrap_or("◆")
                    .style(self.style.summary_prefix_style),
                "Done".style(self.style.summary_text),
            )
            .into_diagnostic()?;
        }

        Ok(())
    }
}
