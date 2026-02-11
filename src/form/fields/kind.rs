use {
    crate::{form::value::FormValue, option::AskOption},
    std::collections::HashSet,
};

pub enum FieldKind {
    Text {
        prompt: String,
        placeholder: Option<String>,
        default: Option<String>,
        help: Option<String>,
        validation: Option<fn(&str) -> Result<(), String>>,
        input: String,
        cursor_pos: usize,
    },
    Confirm {
        prompt: String,
        default: bool,
        yes_text: String,
        no_text: String,
        selected: bool,
    },
    Select {
        prompt: String,
        options: Vec<AskOption>,
        page_size: usize,
        help: Option<String>,
        cursor: usize,
        scroll_offset: usize,
    },
    MultiSelect {
        prompt: String,
        options: Vec<AskOption>,
        page_size: usize,
        min_selections: Option<usize>,
        max_selections: Option<usize>,
        help: Option<String>,
        cursor: usize,
        scroll_offset: usize,
        selected: HashSet<usize>,
    },
    Int {
        prompt: String,
        default: Option<i64>,
        min: Option<i64>,
        max: Option<i64>,
        step: i64,
        help: Option<String>,
        validation: Option<fn(i64) -> Result<(), String>>,
        input: String,
        cursor_pos: usize,
    },
    Float {
        prompt: String,
        default: Option<f64>,
        min: Option<f64>,
        max: Option<f64>,
        step: f64,
        help: Option<String>,
        validation: Option<fn(f64) -> Result<(), String>>,
        input: String,
        cursor_pos: usize,
    },
    Password {
        prompt: String,
        mask_char: char,
        min_length: Option<usize>,
        max_length: Option<usize>,
        help: Option<String>,
        validation: Option<fn(&str) -> Result<(), String>>,
        input: String,
        cursor_pos: usize,
        revealed: bool,
    },
    Sort {
        prompt: String,
        page_size: usize,
        help: Option<String>,
        items: Vec<String>,
        cursor: usize,
        grabbed: bool,
        scroll_offset: usize,
    },
}

impl FieldKind {
    pub fn prompt_text(&self) -> &str {
        match self {
            Self::Text { prompt, .. }
            | Self::Confirm { prompt, .. }
            | Self::Select { prompt, .. }
            | Self::MultiSelect { prompt, .. }
            | Self::Int { prompt, .. }
            | Self::Float { prompt, .. }
            | Self::Password { prompt, .. }
            | Self::Sort { prompt, .. } => prompt,
        }
    }

    pub fn prefix_char(&self) -> &str {
        match self {
            Self::Text { .. }
            | Self::Confirm { .. }
            | Self::Select { .. }
            | Self::MultiSelect { .. } => "?",
            Self::Int { .. } | Self::Float { .. } => "#",
            Self::Password { .. } => "",
            Self::Sort { .. } => "↕",
        }
    }

    pub fn current_value(&self) -> FormValue {
        match self {
            Self::Text { input, default, .. } => {
                let val = if input.is_empty() {
                    default.clone().unwrap_or_default()
                } else {
                    input.clone()
                };
                FormValue::Text(val)
            }
            Self::Confirm { selected, .. } => FormValue::Bool(*selected),
            Self::Select { cursor, .. } => FormValue::Index(*cursor),
            Self::MultiSelect { selected, .. } => {
                let mut v: Vec<usize> = selected.iter().copied().collect();
                v.sort_unstable();
                FormValue::Indices(v)
            }
            Self::Int { input, default, .. } => {
                let val = if input.is_empty() {
                    default.unwrap_or(0)
                } else {
                    input.parse().unwrap_or(0)
                };
                FormValue::Int(val)
            }
            Self::Float { input, default, .. } => {
                let val = if input.is_empty() {
                    default.unwrap_or(0.0)
                } else {
                    input.parse().unwrap_or(0.0)
                };
                FormValue::Float(val)
            }
            Self::Password { input, .. } => FormValue::Text(input.clone()),
            Self::Sort { items, .. } => FormValue::Sorted(items.clone()),
        }
    }

    pub fn value_summary(&self) -> String {
        match self {
            Self::Text { input, default, .. } => {
                if input.is_empty() {
                    default.clone().unwrap_or_default()
                } else {
                    input.clone()
                }
            }
            Self::Confirm {
                selected,
                yes_text,
                no_text,
                ..
            } => {
                if *selected {
                    yes_text.clone()
                } else {
                    no_text.clone()
                }
            }
            Self::Select {
                options, cursor, ..
            } => options
                .get(*cursor)
                .map(|o| o.name.clone())
                .unwrap_or_default(),
            Self::MultiSelect {
                options, selected, ..
            } => {
                let names: Vec<&str> = {
                    let mut indices: Vec<usize> = selected.iter().copied().collect();
                    indices.sort_unstable();
                    indices
                        .iter()
                        .filter_map(|&i| options.get(i).map(|o| o.name.as_str()))
                        .collect()
                };
                if names.is_empty() {
                    "None".into()
                } else {
                    names.join(", ")
                }
            }
            Self::Int { input, default, .. } => {
                if input.is_empty() {
                    default.map(|d| d.to_string()).unwrap_or_default()
                } else {
                    input.clone()
                }
            }
            Self::Float { input, default, .. } => {
                if input.is_empty() {
                    default.map(|d| d.to_string()).unwrap_or_default()
                } else {
                    input.clone()
                }
            }
            Self::Password {
                input, mask_char, ..
            } => {
                if input.is_empty() {
                    String::new()
                } else {
                    mask_char.to_string().repeat(input.len().min(8))
                }
            }
            Self::Sort { items, .. } => items
                .iter()
                .enumerate()
                .map(|(i, s)| format!("{}. {}", i + 1, s))
                .collect::<Vec<_>>()
                .join(", "),
        }
    }
}

pub struct InternalField {
    pub key: String,
    pub section_idx: Option<usize>,
    pub kind: FieldKind,
    pub confirmed: bool,
    pub error: Option<String>,
}
