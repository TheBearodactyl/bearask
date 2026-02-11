use {
    crate::form::{FieldAction, Form, KeyEvent, fields::FieldKind, value::FormValue},
    crossterm::event::{KeyCode, KeyModifiers},
};

impl Form {
    pub fn dispatch_key(&mut self, key: KeyEvent, active: usize) -> FieldAction {
        let is_grabbed = matches!(
            &self.fields[active].kind,
            FieldKind::Sort { grabbed: true, .. }
        );

        if key.code == KeyCode::Tab && !is_grabbed {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                return FieldAction::Prev;
            }
            return FieldAction::Next;
        }

        if key.code == KeyCode::BackTab {
            return FieldAction::Prev;
        }

        if key.code == KeyCode::Esc && self.allow_escape {
            if is_grabbed {
                if let FieldKind::Sort { grabbed, .. } = &mut self.fields[active].kind {
                    *grabbed = false;
                }
                return FieldAction::Continue;
            }
            return FieldAction::Cancel;
        }

        self.fields[active].error = None;

        if key.code != KeyCode::Enter {
            self.fields[active].confirmed = false;
        }

        let field = &mut self.fields[active];
        match &mut field.kind {
            FieldKind::Text { .. } => Self::handle_text_key(key, &mut field.kind, &mut field.error),
            FieldKind::Confirm { .. } => Self::handle_confirm_key(key, &mut field.kind),
            FieldKind::Select { .. } => {
                Self::handle_select_key(key, &mut field.kind, &mut field.error)
            }
            FieldKind::MultiSelect { .. } => {
                Self::handle_multiselect_key(key, &mut field.kind, &mut field.error)
            }
            FieldKind::Int { .. } => Self::handle_int_key(key, &mut field.kind, &mut field.error),
            FieldKind::Float { .. } => {
                Self::handle_float_key(key, &mut field.kind, &mut field.error)
            }
            FieldKind::Password { .. } => {
                Self::handle_password_key(key, &mut field.kind, &mut field.error)
            }
            FieldKind::Sort { .. } => Self::handle_sort_key(key, &mut field.kind, &mut field.error),
        }
    }

    pub fn handle_text_key(
        key: KeyEvent,
        kind: &mut FieldKind,
        error: &mut Option<String>,
    ) -> FieldAction {
        let FieldKind::Text {
            input,
            cursor_pos,
            default,
            validation,
            ..
        } = kind
        else {
            return FieldAction::Continue;
        };

        match key.code {
            KeyCode::Enter => {
                let val = if input.is_empty() {
                    default.clone().unwrap_or_default()
                } else {
                    input.clone()
                };
                if let Some(v) = validation
                    && let Err(e) = v(&val)
                {
                    *error = Some(e);
                    return FieldAction::Continue;
                }
                FormValue::Text(val).into()
            }
            KeyCode::Char(c) => {
                input.insert(*cursor_pos, c);
                *cursor_pos += 1;
                FieldAction::Continue
            }
            KeyCode::Backspace if *cursor_pos > 0 => {
                *cursor_pos -= 1;
                input.remove(*cursor_pos);
                FieldAction::Continue
            }
            KeyCode::Delete if *cursor_pos < input.len() => {
                input.remove(*cursor_pos);
                FieldAction::Continue
            }
            KeyCode::Left if *cursor_pos > 0 => {
                *cursor_pos -= 1;
                FieldAction::Continue
            }
            KeyCode::Right if *cursor_pos < input.len() => {
                *cursor_pos += 1;
                FieldAction::Continue
            }
            KeyCode::Home => {
                *cursor_pos = 0;
                FieldAction::Continue
            }
            KeyCode::End => {
                *cursor_pos = input.len();
                FieldAction::Continue
            }
            _ => FieldAction::Continue,
        }
    }

    pub fn handle_confirm_key(key: KeyEvent, kind: &mut FieldKind) -> FieldAction {
        let FieldKind::Confirm { selected, .. } = kind else {
            return FieldAction::Continue;
        };

        match key.code {
            KeyCode::Enter | KeyCode::Char(' ') => FormValue::Bool(*selected).into(),
            KeyCode::Left | KeyCode::Right => {
                *selected = !*selected;
                FieldAction::Continue
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                *selected = true;
                FieldAction::Continue
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                *selected = false;
                FieldAction::Continue
            }
            _ => FieldAction::Continue,
        }
    }

    pub fn handle_select_key(
        key: KeyEvent,
        kind: &mut FieldKind,
        error: &mut Option<String>,
    ) -> FieldAction {
        let FieldKind::Select {
            options,
            page_size,
            cursor,
            scroll_offset,
            ..
        } = kind
        else {
            return FieldAction::Continue;
        };
        let total = options.len();
        if total == 0 {
            *error = Some("No options".into());
            return FieldAction::Continue;
        }

        match key.code {
            KeyCode::Up => {
                *cursor = if *cursor > 0 { *cursor - 1 } else { total - 1 };
                Self::adjust_scroll(*cursor, scroll_offset, *page_size, total);
                FieldAction::Continue
            }
            KeyCode::Down => {
                *cursor = if *cursor < total - 1 { *cursor + 1 } else { 0 };
                Self::adjust_scroll(*cursor, scroll_offset, *page_size, total);
                FieldAction::Continue
            }
            KeyCode::Enter | KeyCode::Char(' ') => FormValue::Index(*cursor).into(),
            _ => FieldAction::Continue,
        }
    }

    pub fn handle_multiselect_key(
        key: KeyEvent,
        kind: &mut FieldKind,
        error: &mut Option<String>,
    ) -> FieldAction {
        let FieldKind::MultiSelect {
            options,
            page_size,
            min_selections,
            max_selections,
            cursor,
            scroll_offset,
            selected,
            ..
        } = kind
        else {
            return FieldAction::Continue;
        };
        let total = options.len();
        if total == 0 {
            *error = Some("No options".into());
            return FieldAction::Continue;
        }

        match key.code {
            KeyCode::Up => {
                *cursor = if *cursor > 0 { *cursor - 1 } else { total - 1 };
                Self::adjust_scroll(*cursor, scroll_offset, *page_size, total);
                FieldAction::Continue
            }
            KeyCode::Down => {
                *cursor = if *cursor < total - 1 { *cursor + 1 } else { 0 };
                Self::adjust_scroll(*cursor, scroll_offset, *page_size, total);
                FieldAction::Continue
            }
            KeyCode::Char(' ') => {
                if selected.contains(cursor) {
                    selected.remove(cursor);
                } else {
                    if let Some(max) = max_selections
                        && selected.len() >= *max
                    {
                        return FieldAction::Continue;
                    }
                    selected.insert(*cursor);
                }
                FieldAction::Continue
            }
            KeyCode::Right => {
                if let Some(max) = max_selections
                    && total > *max
                {
                    return FieldAction::Continue;
                }
                *selected = (0..total).collect();
                FieldAction::Continue
            }
            KeyCode::Left => {
                selected.clear();
                FieldAction::Continue
            }
            KeyCode::Enter => {
                if let Some(min) = min_selections
                    && selected.len() < *min
                {
                    *error = Some(format!("Select at least {}", min));
                    return FieldAction::Continue;
                }
                if let Some(max) = max_selections
                    && selected.len() > *max
                {
                    *error = Some(format!("Select at most {}", max));
                    return FieldAction::Continue;
                }
                let mut v: Vec<usize> = selected.iter().copied().collect();
                v.sort_unstable();
                FormValue::Indices(v).into()
            }
            _ => FieldAction::Continue,
        }
    }

    pub fn handle_int_key(
        key: KeyEvent,
        kind: &mut FieldKind,
        error: &mut Option<String>,
    ) -> FieldAction {
        let FieldKind::Int {
            input,
            cursor_pos,
            default,
            min,
            max,
            step,
            validation,
            ..
        } = kind
        else {
            return FieldAction::Continue;
        };

        match key.code {
            KeyCode::Enter => {
                let val: i64 = if input.is_empty() {
                    match default {
                        Some(d) => *d,
                        None => {
                            *error = Some("Enter a number".into());
                            return FieldAction::Continue;
                        }
                    }
                } else {
                    match input.parse() {
                        Ok(v) => v,
                        Err(_) => {
                            *error = Some(format!("Invalid number: {}", input));
                            return FieldAction::Continue;
                        }
                    }
                };

                if let Some(lo) = min
                    && val < *lo
                {
                    *error = Some(format!("Must be at least {}", lo));
                    return FieldAction::Continue;
                }

                if let Some(hi) = max
                    && val > *hi
                {
                    *error = Some(format!("Must be at most {}", hi));
                    return FieldAction::Continue;
                }

                if let Some(v) = validation
                    && let Err(e) = v(val)
                {
                    *error = Some(e);
                    return FieldAction::Continue;
                }
                FormValue::Int(val).into()
            }
            KeyCode::Up => {
                if let Ok(cur) = input.parse::<i64>() {
                    let nv = cur + *step;
                    if max.is_none() || nv <= max.unwrap() {
                        *input = nv.to_string();
                        *cursor_pos = input.len();
                    }
                } else if let Some(d) = default {
                    *input = d.to_string();
                    *cursor_pos = input.len();
                }
                FieldAction::Continue
            }
            KeyCode::Down => {
                if let Ok(cur) = input.parse::<i64>() {
                    let nv = cur - *step;
                    if min.is_none() || nv >= min.unwrap() {
                        *input = nv.to_string();
                        *cursor_pos = input.len();
                    }
                } else if let Some(d) = default {
                    *input = d.to_string();
                    *cursor_pos = input.len();
                }
                FieldAction::Continue
            }
            KeyCode::Char(c) if c.is_ascii_digit() || c == '-' => {
                input.insert(*cursor_pos, c);
                *cursor_pos += 1;
                FieldAction::Continue
            }
            KeyCode::Backspace if *cursor_pos > 0 => {
                *cursor_pos -= 1;
                input.remove(*cursor_pos);
                FieldAction::Continue
            }
            KeyCode::Left if *cursor_pos > 0 => {
                *cursor_pos -= 1;
                FieldAction::Continue
            }
            KeyCode::Right if *cursor_pos < input.len() => {
                *cursor_pos += 1;
                FieldAction::Continue
            }
            KeyCode::Home => {
                *cursor_pos = 0;
                FieldAction::Continue
            }
            KeyCode::End => {
                *cursor_pos = input.len();
                FieldAction::Continue
            }
            _ => FieldAction::Continue,
        }
    }

    pub fn handle_float_key(
        key: KeyEvent,
        kind: &mut FieldKind,
        error: &mut Option<String>,
    ) -> FieldAction {
        let FieldKind::Float {
            input,
            cursor_pos,
            default,
            min,
            max,
            step,
            validation,
            ..
        } = kind
        else {
            return FieldAction::Continue;
        };

        match key.code {
            KeyCode::Enter => {
                let val: f64 = if input.is_empty() {
                    match default {
                        Some(d) => *d,
                        None => {
                            *error = Some("Enter a number".into());
                            return FieldAction::Continue;
                        }
                    }
                } else {
                    match input.parse() {
                        Ok(v) => v,
                        Err(_) => {
                            *error = Some(format!("Invalid number: {}", input));
                            return FieldAction::Continue;
                        }
                    }
                };

                if let Some(lo) = min
                    && val < *lo
                {
                    *error = Some(format!("Must be at least {}", lo));
                    return FieldAction::Continue;
                }

                if let Some(hi) = max
                    && val > *hi
                {
                    *error = Some(format!("Must be at most {}", hi));
                    return FieldAction::Continue;
                }

                if let Some(v) = validation
                    && let Err(e) = v(val)
                {
                    *error = Some(e);
                    return FieldAction::Continue;
                }
                FormValue::Float(val).into()
            }
            KeyCode::Up => {
                if let Ok(cur) = input.parse::<f64>() {
                    let nv = cur + *step;
                    if max.is_none() || nv <= max.unwrap() {
                        *input = nv.to_string();
                        *cursor_pos = input.len();
                    }
                } else if let Some(d) = default {
                    *input = d.to_string();
                    *cursor_pos = input.len();
                }
                FieldAction::Continue
            }
            KeyCode::Down => {
                if let Ok(cur) = input.parse::<f64>() {
                    let nv = cur - *step;
                    if min.is_none() || nv >= min.unwrap() {
                        *input = nv.to_string();
                        *cursor_pos = input.len();
                    }
                } else if let Some(d) = default {
                    *input = d.to_string();
                    *cursor_pos = input.len();
                }
                FieldAction::Continue
            }
            KeyCode::Char(c) if c.is_ascii_digit() || c == '-' || c == '.' => {
                input.insert(*cursor_pos, c);
                *cursor_pos += 1;
                FieldAction::Continue
            }
            KeyCode::Backspace if *cursor_pos > 0 => {
                *cursor_pos -= 1;
                input.remove(*cursor_pos);
                FieldAction::Continue
            }
            KeyCode::Left if *cursor_pos > 0 => {
                *cursor_pos -= 1;
                FieldAction::Continue
            }
            KeyCode::Right if *cursor_pos < input.len() => {
                *cursor_pos += 1;
                FieldAction::Continue
            }
            KeyCode::Home => {
                *cursor_pos = 0;
                FieldAction::Continue
            }
            KeyCode::End => {
                *cursor_pos = input.len();
                FieldAction::Continue
            }
            _ => FieldAction::Continue,
        }
    }

    pub fn handle_password_key(
        key: KeyEvent,
        kind: &mut FieldKind,
        error: &mut Option<String>,
    ) -> FieldAction {
        let FieldKind::Password {
            input,
            cursor_pos,
            min_length,
            max_length,
            validation,
            revealed,
            ..
        } = kind
        else {
            return FieldAction::Continue;
        };

        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('r') {
            *revealed = !*revealed;
            return FieldAction::Continue;
        }

        match key.code {
            KeyCode::Enter => {
                if let Some(min) = min_length
                    && input.len() < *min
                {
                    *error = Some(format!("At least {} characters", min));
                    return FieldAction::Continue;
                }
                if let Some(max) = max_length
                    && input.len() > *max
                {
                    *error = Some(format!("At most {} characters", max));
                    return FieldAction::Continue;
                }
                if let Some(v) = validation
                    && let Err(e) = v(input)
                {
                    *error = Some(e);
                    return FieldAction::Continue;
                }
                FormValue::Text(input.clone()).into()
            }
            KeyCode::Char(c) => {
                input.insert(*cursor_pos, c);
                *cursor_pos += 1;
                FieldAction::Continue
            }
            KeyCode::Backspace if *cursor_pos > 0 => {
                *cursor_pos -= 1;
                input.remove(*cursor_pos);
                FieldAction::Continue
            }
            KeyCode::Delete if *cursor_pos < input.len() => {
                input.remove(*cursor_pos);
                FieldAction::Continue
            }
            KeyCode::Left if *cursor_pos > 0 => {
                *cursor_pos -= 1;
                FieldAction::Continue
            }
            KeyCode::Right if *cursor_pos < input.len() => {
                *cursor_pos += 1;
                FieldAction::Continue
            }
            KeyCode::Home => {
                *cursor_pos = 0;
                FieldAction::Continue
            }
            KeyCode::End => {
                *cursor_pos = input.len();
                FieldAction::Continue
            }
            _ => FieldAction::Continue,
        }
    }

    pub fn handle_sort_key(
        key: KeyEvent,
        kind: &mut FieldKind,
        error: &mut Option<String>,
    ) -> FieldAction {
        let FieldKind::Sort {
            page_size,
            items,
            cursor,
            grabbed,
            scroll_offset,
            ..
        } = kind
        else {
            return FieldAction::Continue;
        };
        let total = items.len();
        if total == 0 {
            *error = Some("No items".into());
            return FieldAction::Continue;
        }

        match key.code {
            KeyCode::Up => {
                if *grabbed && *cursor > 0 {
                    items.swap(*cursor, *cursor - 1);
                    *cursor -= 1;
                } else if !*grabbed {
                    *cursor = if *cursor > 0 { *cursor - 1 } else { total - 1 };
                }
                Self::adjust_scroll(*cursor, scroll_offset, *page_size, total);
                FieldAction::Continue
            }
            KeyCode::Down => {
                if *grabbed && *cursor < total - 1 {
                    items.swap(*cursor, *cursor + 1);
                    *cursor += 1;
                } else if !*grabbed {
                    *cursor = if *cursor < total - 1 { *cursor + 1 } else { 0 };
                }
                Self::adjust_scroll(*cursor, scroll_offset, *page_size, total);
                FieldAction::Continue
            }
            KeyCode::Char(' ') => {
                *grabbed = !*grabbed;
                FieldAction::Continue
            }
            KeyCode::Enter => {
                if *grabbed {
                    *grabbed = false;
                    return FieldAction::Continue;
                }
                FormValue::Sorted(items.clone()).into()
            }
            _ => FieldAction::Continue,
        }
    }
}
