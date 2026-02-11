use owo_colors::{Style, colors::CustomColor};

#[derive(Clone)]
pub struct ConfirmStyle {
    pub prompt: Style,
    pub prompt_prefix: Style,
    pub hint: Style,
    pub yes_style: Style,
    pub no_style: Style,
    pub selected: Style,
    pub default_value: Style,
    pub error: Style,
    pub error_hint: Style,
    pub error_prefix: Option<String>,
}

impl Default for ConfirmStyle {
    fn default() -> Self {
        Self::mocha()
    }
}

impl ConfirmStyle {
    pub fn mocha() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<205, 214, 244>>(),
            prompt_prefix: Style::new().fg::<CustomColor<137, 180, 250>>(),
            hint: Style::new().fg::<CustomColor<127, 132, 156>>(),
            yes_style: Style::new().fg::<CustomColor<166, 227, 161>>(),
            no_style: Style::new().fg::<CustomColor<243, 139, 168>>(),
            selected: Style::new().fg::<CustomColor<137, 180, 250>>().bold(),
            default_value: Style::new().fg::<CustomColor<186, 194, 222>>().italic(),
            error: Style::new().fg::<CustomColor<243, 139, 168>>().bold(),
            error_hint: Style::new().fg::<CustomColor<147, 153, 178>>(),
            error_prefix: Some("✗".into()),
        }
    }

    pub fn frappe() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<198, 208, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<140, 170, 238>>(),
            hint: Style::new().fg::<CustomColor<131, 139, 167>>(),
            yes_style: Style::new().fg::<CustomColor<166, 209, 137>>(),
            no_style: Style::new().fg::<CustomColor<231, 130, 132>>(),
            selected: Style::new().fg::<CustomColor<140, 170, 238>>().bold(),
            default_value: Style::new().fg::<CustomColor<181, 191, 226>>().italic(),
            error: Style::new().fg::<CustomColor<231, 130, 132>>().bold(),
            error_hint: Style::new().fg::<CustomColor<165, 173, 206>>(),
            error_prefix: Some("✗".into()),
        }
    }

    pub fn macchiato() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<202, 211, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<138, 173, 244>>(),
            hint: Style::new().fg::<CustomColor<128, 135, 162>>(),
            yes_style: Style::new().fg::<CustomColor<166, 218, 149>>(),
            no_style: Style::new().fg::<CustomColor<237, 135, 150>>(),
            selected: Style::new().fg::<CustomColor<138, 173, 244>>().bold(),
            default_value: Style::new().fg::<CustomColor<184, 192, 224>>().italic(),
            error: Style::new().fg::<CustomColor<237, 135, 150>>().bold(),
            error_hint: Style::new().fg::<CustomColor<153, 160, 187>>(),
            error_prefix: Some("✗".into()),
        }
    }

    pub fn latte() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<76, 79, 105>>(),
            prompt_prefix: Style::new().fg::<CustomColor<30, 102, 245>>(),
            hint: Style::new().fg::<CustomColor<156, 160, 176>>(),
            yes_style: Style::new().fg::<CustomColor<64, 160, 43>>(),
            no_style: Style::new().fg::<CustomColor<210, 15, 57>>(),
            selected: Style::new().fg::<CustomColor<30, 102, 245>>().bold(),
            default_value: Style::new().fg::<CustomColor<92, 95, 119>>().italic(),
            error: Style::new().fg::<CustomColor<210, 15, 57>>().bold(),
            error_hint: Style::new().fg::<CustomColor<124, 127, 147>>(),
            error_prefix: Some("✗".into()),
        }
    }

    pub fn minimal() -> Self {
        Self {
            prompt: Style::new(),
            prompt_prefix: Style::new().bold(),
            hint: Style::new().dimmed(),
            yes_style: Style::new(),
            no_style: Style::new(),
            selected: Style::new().bold(),
            default_value: Style::new().dimmed(),
            error: Style::new().bold(),
            error_hint: Style::new().dimmed(),
            error_prefix: Some("error:".into()),
        }
    }
}

#[derive(Clone)]
pub struct TextInputStyle {
    pub prompt: Style,
    pub prompt_prefix: Style,
    pub hint: Style,
    pub input: Style,
    pub placeholder: Style,
    pub suggestion: Style,
    pub selected: Style,
    pub default_value: Style,
    pub error: Style,
    pub error_hint: Style,
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self::mocha()
    }
}

impl TextInputStyle {
    pub fn mocha() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<205, 214, 244>>(),
            prompt_prefix: Style::new().fg::<CustomColor<137, 180, 250>>(),
            hint: Style::new().fg::<CustomColor<127, 132, 156>>(),
            input: Style::new().fg::<CustomColor<166, 227, 161>>(),
            placeholder: Style::new().fg::<CustomColor<127, 132, 156>>().italic(),
            suggestion: Style::new().fg::<CustomColor<186, 194, 222>>(),
            selected: Style::new().fg::<CustomColor<137, 180, 250>>().bold(),
            default_value: Style::new().fg::<CustomColor<186, 194, 222>>().italic(),
            error: Style::new().fg::<CustomColor<243, 139, 168>>().bold(),
            error_hint: Style::new().fg::<CustomColor<147, 153, 178>>(),
        }
    }

    pub fn frappe() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<198, 208, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<140, 170, 238>>(),
            hint: Style::new().fg::<CustomColor<131, 139, 167>>(),
            input: Style::new().fg::<CustomColor<166, 209, 137>>(),
            placeholder: Style::new().fg::<CustomColor<131, 139, 167>>().italic(),
            suggestion: Style::new().fg::<CustomColor<181, 191, 226>>(),
            selected: Style::new().fg::<CustomColor<140, 170, 238>>().bold(),
            default_value: Style::new().fg::<CustomColor<181, 191, 226>>().italic(),
            error: Style::new().fg::<CustomColor<231, 130, 132>>().bold(),
            error_hint: Style::new().fg::<CustomColor<165, 173, 206>>(),
        }
    }

    pub fn macchiato() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<202, 211, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<138, 173, 244>>(),
            hint: Style::new().fg::<CustomColor<128, 135, 162>>(),
            input: Style::new().fg::<CustomColor<166, 218, 149>>(),
            placeholder: Style::new().fg::<CustomColor<128, 135, 162>>().italic(),
            suggestion: Style::new().fg::<CustomColor<184, 192, 224>>(),
            selected: Style::new().fg::<CustomColor<138, 173, 244>>().bold(),
            default_value: Style::new().fg::<CustomColor<184, 192, 224>>().italic(),
            error: Style::new().fg::<CustomColor<237, 135, 150>>().bold(),
            error_hint: Style::new().fg::<CustomColor<153, 160, 187>>(),
        }
    }

    pub fn latte() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<76, 79, 105>>(),
            prompt_prefix: Style::new().fg::<CustomColor<30, 102, 245>>(),
            hint: Style::new().fg::<CustomColor<156, 160, 176>>(),
            input: Style::new().fg::<CustomColor<64, 160, 43>>(),
            placeholder: Style::new().fg::<CustomColor<156, 160, 176>>().italic(),
            suggestion: Style::new().fg::<CustomColor<92, 95, 119>>(),
            selected: Style::new().fg::<CustomColor<30, 102, 245>>().bold(),
            default_value: Style::new().fg::<CustomColor<92, 95, 119>>().italic(),
            error: Style::new().fg::<CustomColor<210, 15, 57>>().bold(),
            error_hint: Style::new().fg::<CustomColor<124, 127, 147>>(),
        }
    }

    pub fn minimal() -> Self {
        Self {
            prompt: Style::new(),
            prompt_prefix: Style::new().bold(),
            hint: Style::new().dimmed(),
            input: Style::new(),
            placeholder: Style::new().dimmed().italic(),
            suggestion: Style::new().dimmed(),
            selected: Style::new().bold(),
            default_value: Style::new().dimmed(),
            error: Style::new().bold(),
            error_hint: Style::new().dimmed(),
        }
    }
}

#[derive(Clone)]
pub struct PasswordStyle {
    pub prompt: Style,
    pub prompt_prefix: Style,
    pub hint: Style,
    pub input_masked: Style,
    pub input_revealed: Style,
    pub strength_weak: Style,
    pub strength_medium: Style,
    pub strength_strong: Style,
    pub error: Style,
    pub error_hint: Style,
}

impl Default for PasswordStyle {
    fn default() -> Self {
        Self::mocha()
    }
}

impl PasswordStyle {
    pub fn mocha() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<205, 214, 244>>(),
            prompt_prefix: Style::new().fg::<CustomColor<137, 180, 250>>(),
            hint: Style::new().fg::<CustomColor<127, 132, 156>>(),
            input_masked: Style::new().fg::<CustomColor<186, 194, 222>>(),
            input_revealed: Style::new().fg::<CustomColor<166, 227, 161>>(),
            strength_weak: Style::new().fg::<CustomColor<243, 139, 168>>(),
            strength_medium: Style::new().fg::<CustomColor<249, 226, 175>>(),
            strength_strong: Style::new().fg::<CustomColor<166, 227, 161>>(),
            error: Style::new().fg::<CustomColor<243, 139, 168>>().bold(),
            error_hint: Style::new().fg::<CustomColor<147, 153, 178>>(),
        }
    }

    pub fn frappe() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<198, 208, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<140, 170, 238>>(),
            hint: Style::new().fg::<CustomColor<131, 139, 167>>(),
            input_masked: Style::new().fg::<CustomColor<181, 191, 226>>(),
            input_revealed: Style::new().fg::<CustomColor<166, 209, 137>>(),
            strength_weak: Style::new().fg::<CustomColor<231, 130, 132>>(),
            strength_medium: Style::new().fg::<CustomColor<229, 200, 144>>(),
            strength_strong: Style::new().fg::<CustomColor<166, 209, 137>>(),
            error: Style::new().fg::<CustomColor<231, 130, 132>>().bold(),
            error_hint: Style::new().fg::<CustomColor<165, 173, 206>>(),
        }
    }

    pub fn macchiato() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<202, 211, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<138, 173, 244>>(),
            hint: Style::new().fg::<CustomColor<128, 135, 162>>(),
            input_masked: Style::new().fg::<CustomColor<184, 192, 224>>(),
            input_revealed: Style::new().fg::<CustomColor<166, 218, 149>>(),
            strength_weak: Style::new().fg::<CustomColor<237, 135, 150>>(),
            strength_medium: Style::new().fg::<CustomColor<238, 212, 159>>(),
            strength_strong: Style::new().fg::<CustomColor<166, 218, 149>>(),
            error: Style::new().fg::<CustomColor<237, 135, 150>>().bold(),
            error_hint: Style::new().fg::<CustomColor<153, 160, 187>>(),
        }
    }

    pub fn latte() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<76, 79, 105>>(),
            prompt_prefix: Style::new().fg::<CustomColor<30, 102, 245>>(),
            hint: Style::new().fg::<CustomColor<156, 160, 176>>(),
            input_masked: Style::new().fg::<CustomColor<92, 95, 119>>(),
            input_revealed: Style::new().fg::<CustomColor<64, 160, 43>>(),
            strength_weak: Style::new().fg::<CustomColor<210, 15, 57>>(),
            strength_medium: Style::new().fg::<CustomColor<223, 142, 29>>(),
            strength_strong: Style::new().fg::<CustomColor<64, 160, 43>>(),
            error: Style::new().fg::<CustomColor<210, 15, 57>>().bold(),
            error_hint: Style::new().fg::<CustomColor<124, 127, 147>>(),
        }
    }

    pub fn minimal() -> Self {
        Self {
            prompt: Style::new(),
            prompt_prefix: Style::new().bold(),
            hint: Style::new().dimmed(),
            input_masked: Style::new().dimmed(),
            input_revealed: Style::new(),
            strength_weak: Style::new().bold(),
            strength_medium: Style::new().bold(),
            strength_strong: Style::new().bold(),
            error: Style::new().bold(),
            error_hint: Style::new().dimmed(),
        }
    }
}

#[derive(Clone)]
pub struct NumberStyle {
    pub prompt: Style,
    pub prompt_prefix: Style,
    pub hint: Style,
    pub input: Style,
    pub bounds: Style,
    pub default_value: Style,
    pub error: Style,
    pub error_hint: Style,
}

impl Default for NumberStyle {
    fn default() -> Self {
        Self::mocha()
    }
}

impl NumberStyle {
    pub fn mocha() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<205, 214, 244>>(),
            prompt_prefix: Style::new().fg::<CustomColor<137, 180, 250>>(),
            hint: Style::new().fg::<CustomColor<127, 132, 156>>(),
            input: Style::new().fg::<CustomColor<166, 227, 161>>(),
            bounds: Style::new().fg::<CustomColor<147, 153, 178>>().italic(),
            default_value: Style::new().fg::<CustomColor<186, 194, 222>>().italic(),
            error: Style::new().fg::<CustomColor<243, 139, 168>>().bold(),
            error_hint: Style::new().fg::<CustomColor<147, 153, 178>>(),
        }
    }

    pub fn frappe() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<198, 208, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<140, 170, 238>>(),
            hint: Style::new().fg::<CustomColor<131, 139, 167>>(),
            input: Style::new().fg::<CustomColor<166, 209, 137>>(),
            bounds: Style::new().fg::<CustomColor<165, 173, 206>>().italic(),
            default_value: Style::new().fg::<CustomColor<181, 191, 226>>().italic(),
            error: Style::new().fg::<CustomColor<231, 130, 132>>().bold(),
            error_hint: Style::new().fg::<CustomColor<165, 173, 206>>(),
        }
    }

    pub fn macchiato() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<202, 211, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<138, 173, 244>>(),
            hint: Style::new().fg::<CustomColor<128, 135, 162>>(),
            input: Style::new().fg::<CustomColor<166, 218, 149>>(),
            bounds: Style::new().fg::<CustomColor<153, 160, 187>>().italic(),
            default_value: Style::new().fg::<CustomColor<184, 192, 224>>().italic(),
            error: Style::new().fg::<CustomColor<237, 135, 150>>().bold(),
            error_hint: Style::new().fg::<CustomColor<153, 160, 187>>(),
        }
    }

    pub fn latte() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<76, 79, 105>>(),
            prompt_prefix: Style::new().fg::<CustomColor<30, 102, 245>>(),
            hint: Style::new().fg::<CustomColor<156, 160, 176>>(),
            input: Style::new().fg::<CustomColor<64, 160, 43>>(),
            bounds: Style::new().fg::<CustomColor<124, 127, 147>>().italic(),
            default_value: Style::new().fg::<CustomColor<92, 95, 119>>().italic(),
            error: Style::new().fg::<CustomColor<210, 15, 57>>().bold(),
            error_hint: Style::new().fg::<CustomColor<124, 127, 147>>(),
        }
    }

    pub fn minimal() -> Self {
        Self {
            prompt: Style::new(),
            prompt_prefix: Style::new().bold(),
            hint: Style::new().dimmed(),
            input: Style::new(),
            bounds: Style::new().dimmed(),
            default_value: Style::new().dimmed(),
            error: Style::new().bold(),
            error_hint: Style::new().dimmed(),
        }
    }
}

#[derive(Clone)]
pub struct SelectStyle {
    pub prompt: Style,
    pub prompt_prefix: Style,
    pub hint: Style,
    pub option_name: Style,
    pub option_description: Style,
    pub selected: Style,
    pub selected_description: Style,
    pub error: Style,
    pub error_hint: Style,
}

impl Default for SelectStyle {
    fn default() -> Self {
        Self::mocha()
    }
}

impl SelectStyle {
    pub fn mocha() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<205, 214, 244>>(),
            prompt_prefix: Style::new().fg::<CustomColor<137, 180, 250>>(),
            hint: Style::new().fg::<CustomColor<127, 132, 156>>(),
            option_name: Style::new().fg::<CustomColor<186, 194, 222>>(),
            option_description: Style::new().fg::<CustomColor<147, 153, 178>>(),
            selected: Style::new().fg::<CustomColor<137, 180, 250>>().bold(),
            selected_description: Style::new().fg::<CustomColor<166, 227, 161>>(),
            error: Style::new().fg::<CustomColor<243, 139, 168>>().bold(),
            error_hint: Style::new().fg::<CustomColor<147, 153, 178>>(),
        }
    }

    pub fn frappe() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<198, 208, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<140, 170, 238>>(),
            hint: Style::new().fg::<CustomColor<131, 139, 167>>(),
            option_name: Style::new().fg::<CustomColor<181, 191, 226>>(),
            option_description: Style::new().fg::<CustomColor<165, 173, 206>>(),
            selected: Style::new().fg::<CustomColor<140, 170, 238>>().bold(),
            selected_description: Style::new().fg::<CustomColor<166, 209, 137>>(),
            error: Style::new().fg::<CustomColor<231, 130, 132>>().bold(),
            error_hint: Style::new().fg::<CustomColor<165, 173, 206>>(),
        }
    }

    pub fn macchiato() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<202, 211, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<138, 173, 244>>(),
            hint: Style::new().fg::<CustomColor<128, 135, 162>>(),
            option_name: Style::new().fg::<CustomColor<184, 192, 224>>(),
            option_description: Style::new().fg::<CustomColor<153, 160, 187>>(),
            selected: Style::new().fg::<CustomColor<138, 173, 244>>().bold(),
            selected_description: Style::new().fg::<CustomColor<166, 218, 149>>(),
            error: Style::new().fg::<CustomColor<237, 135, 150>>().bold(),
            error_hint: Style::new().fg::<CustomColor<153, 160, 187>>(),
        }
    }

    pub fn latte() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<76, 79, 105>>(),
            prompt_prefix: Style::new().fg::<CustomColor<30, 102, 245>>(),
            hint: Style::new().fg::<CustomColor<156, 160, 176>>(),
            option_name: Style::new().fg::<CustomColor<92, 95, 119>>(),
            option_description: Style::new().fg::<CustomColor<124, 127, 147>>(),
            selected: Style::new().fg::<CustomColor<30, 102, 245>>().bold(),
            selected_description: Style::new().fg::<CustomColor<64, 160, 43>>(),
            error: Style::new().fg::<CustomColor<210, 15, 57>>().bold(),
            error_hint: Style::new().fg::<CustomColor<124, 127, 147>>(),
        }
    }

    pub fn minimal() -> Self {
        Self {
            prompt: Style::new(),
            prompt_prefix: Style::new().bold(),
            hint: Style::new().dimmed(),
            option_name: Style::new(),
            option_description: Style::new().dimmed(),
            selected: Style::new().bold(),
            selected_description: Style::new(),
            error: Style::new().bold(),
            error_hint: Style::new().dimmed(),
        }
    }
}

#[derive(Clone)]
pub struct MultiSelectStyle {
    pub prompt: Style,
    pub prompt_prefix: Style,
    pub hint: Style,
    pub option_name: Style,
    pub option_description: Style,
    pub cursor: Style,
    pub cursor_description: Style,
    pub selected: Style,
    pub checkbox_selected: Style,
    pub checkbox_unselected: Style,
    pub selection_count: Style,
    pub error: Style,
    pub error_hint: Style,
}

impl Default for MultiSelectStyle {
    fn default() -> Self {
        Self::mocha()
    }
}

impl MultiSelectStyle {
    pub fn mocha() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<205, 214, 244>>(),
            prompt_prefix: Style::new().fg::<CustomColor<137, 180, 250>>(),
            hint: Style::new().fg::<CustomColor<127, 132, 156>>(),
            option_name: Style::new().fg::<CustomColor<186, 194, 222>>(),
            option_description: Style::new().fg::<CustomColor<147, 153, 178>>(),
            cursor: Style::new().fg::<CustomColor<137, 180, 250>>().bold(),
            cursor_description: Style::new().fg::<CustomColor<166, 227, 161>>(),
            selected: Style::new().fg::<CustomColor<166, 227, 161>>(),
            checkbox_selected: Style::new().fg::<CustomColor<166, 227, 161>>(),
            checkbox_unselected: Style::new().fg::<CustomColor<127, 132, 156>>(),
            selection_count: Style::new().fg::<CustomColor<186, 194, 222>>().dimmed(),
            error: Style::new().fg::<CustomColor<243, 139, 168>>().bold(),
            error_hint: Style::new().fg::<CustomColor<147, 153, 178>>(),
        }
    }

    pub fn frappe() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<198, 208, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<140, 170, 238>>(),
            hint: Style::new().fg::<CustomColor<131, 139, 167>>(),
            option_name: Style::new().fg::<CustomColor<181, 191, 226>>(),
            option_description: Style::new().fg::<CustomColor<165, 173, 206>>(),
            cursor: Style::new().fg::<CustomColor<140, 170, 238>>().bold(),
            cursor_description: Style::new().fg::<CustomColor<166, 209, 137>>(),
            selected: Style::new().fg::<CustomColor<166, 209, 137>>(),
            checkbox_selected: Style::new().fg::<CustomColor<166, 209, 137>>(),
            checkbox_unselected: Style::new().fg::<CustomColor<131, 139, 167>>(),
            selection_count: Style::new().fg::<CustomColor<181, 191, 226>>().dimmed(),
            error: Style::new().fg::<CustomColor<231, 130, 132>>().bold(),
            error_hint: Style::new().fg::<CustomColor<165, 173, 206>>(),
        }
    }

    pub fn macchiato() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<202, 211, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<138, 173, 244>>(),
            hint: Style::new().fg::<CustomColor<128, 135, 162>>(),
            option_name: Style::new().fg::<CustomColor<184, 192, 224>>(),
            option_description: Style::new().fg::<CustomColor<153, 160, 187>>(),
            cursor: Style::new().fg::<CustomColor<138, 173, 244>>().bold(),
            cursor_description: Style::new().fg::<CustomColor<166, 218, 149>>(),
            selected: Style::new().fg::<CustomColor<166, 218, 149>>(),
            checkbox_selected: Style::new().fg::<CustomColor<166, 218, 149>>(),
            checkbox_unselected: Style::new().fg::<CustomColor<128, 135, 162>>(),
            selection_count: Style::new().fg::<CustomColor<184, 192, 224>>().dimmed(),
            error: Style::new().fg::<CustomColor<237, 135, 150>>().bold(),
            error_hint: Style::new().fg::<CustomColor<153, 160, 187>>(),
        }
    }

    pub fn latte() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<76, 79, 105>>(),
            prompt_prefix: Style::new().fg::<CustomColor<30, 102, 245>>(),
            hint: Style::new().fg::<CustomColor<156, 160, 176>>(),
            option_name: Style::new().fg::<CustomColor<92, 95, 119>>(),
            option_description: Style::new().fg::<CustomColor<124, 127, 147>>(),
            cursor: Style::new().fg::<CustomColor<30, 102, 245>>().bold(),
            cursor_description: Style::new().fg::<CustomColor<64, 160, 43>>(),
            selected: Style::new().fg::<CustomColor<64, 160, 43>>(),
            checkbox_selected: Style::new().fg::<CustomColor<64, 160, 43>>(),
            checkbox_unselected: Style::new().fg::<CustomColor<156, 160, 176>>(),
            selection_count: Style::new().fg::<CustomColor<92, 95, 119>>().dimmed(),
            error: Style::new().fg::<CustomColor<210, 15, 57>>().bold(),
            error_hint: Style::new().fg::<CustomColor<124, 127, 147>>(),
        }
    }

    pub fn minimal() -> Self {
        Self {
            prompt: Style::new(),
            prompt_prefix: Style::new().bold(),
            hint: Style::new().dimmed(),
            option_name: Style::new(),
            option_description: Style::new().dimmed(),
            cursor: Style::new().bold(),
            cursor_description: Style::new(),
            selected: Style::new(),
            checkbox_selected: Style::new(),
            checkbox_unselected: Style::new().dimmed(),
            selection_count: Style::new().dimmed(),
            error: Style::new().bold(),
            error_hint: Style::new().dimmed(),
        }
    }
}

#[derive(Clone)]
pub struct SortStyle {
    pub prompt: Style,
    pub prompt_prefix: Style,
    pub hint: Style,
    pub item: Style,
    pub cursor: Style,
    pub grabbed: Style,
    pub index: Style,
    pub error: Style,
    pub error_hint: Style,
}

impl Default for SortStyle {
    fn default() -> Self {
        Self::mocha()
    }
}

impl SortStyle {
    pub fn mocha() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<205, 214, 244>>(),
            prompt_prefix: Style::new().fg::<CustomColor<137, 180, 250>>(),
            hint: Style::new().fg::<CustomColor<127, 132, 156>>(),
            item: Style::new().fg::<CustomColor<186, 194, 222>>(),
            cursor: Style::new().fg::<CustomColor<137, 180, 250>>().bold(),
            grabbed: Style::new().fg::<CustomColor<166, 227, 161>>().bold(),
            index: Style::new().fg::<CustomColor<147, 153, 178>>().dimmed(),
            error: Style::new().fg::<CustomColor<243, 139, 168>>().bold(),
            error_hint: Style::new().fg::<CustomColor<147, 153, 178>>(),
        }
    }

    pub fn frappe() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<198, 208, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<140, 170, 238>>(),
            hint: Style::new().fg::<CustomColor<131, 139, 167>>(),
            item: Style::new().fg::<CustomColor<181, 191, 226>>(),
            cursor: Style::new().fg::<CustomColor<140, 170, 238>>().bold(),
            grabbed: Style::new().fg::<CustomColor<166, 209, 137>>().bold(),
            index: Style::new().fg::<CustomColor<165, 173, 206>>().dimmed(),
            error: Style::new().fg::<CustomColor<231, 130, 132>>().bold(),
            error_hint: Style::new().fg::<CustomColor<165, 173, 206>>(),
        }
    }

    pub fn macchiato() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<202, 211, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<138, 173, 244>>(),
            hint: Style::new().fg::<CustomColor<128, 135, 162>>(),
            item: Style::new().fg::<CustomColor<184, 192, 224>>(),
            cursor: Style::new().fg::<CustomColor<138, 173, 244>>().bold(),
            grabbed: Style::new().fg::<CustomColor<166, 218, 149>>().bold(),
            index: Style::new().fg::<CustomColor<153, 160, 187>>().dimmed(),
            error: Style::new().fg::<CustomColor<237, 135, 150>>().bold(),
            error_hint: Style::new().fg::<CustomColor<153, 160, 187>>(),
        }
    }

    pub fn latte() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<76, 79, 105>>(),
            prompt_prefix: Style::new().fg::<CustomColor<30, 102, 245>>(),
            hint: Style::new().fg::<CustomColor<156, 160, 176>>(),
            item: Style::new().fg::<CustomColor<92, 95, 119>>(),
            cursor: Style::new().fg::<CustomColor<30, 102, 245>>().bold(),
            grabbed: Style::new().fg::<CustomColor<64, 160, 43>>().bold(),
            index: Style::new().fg::<CustomColor<124, 127, 147>>().dimmed(),
            error: Style::new().fg::<CustomColor<210, 15, 57>>().bold(),
            error_hint: Style::new().fg::<CustomColor<124, 127, 147>>(),
        }
    }

    pub fn minimal() -> Self {
        Self {
            prompt: Style::new(),
            prompt_prefix: Style::new().bold(),
            hint: Style::new().dimmed(),
            item: Style::new(),
            cursor: Style::new().bold(),
            grabbed: Style::new().bold(),
            index: Style::new().dimmed(),
            error: Style::new().bold(),
            error_hint: Style::new().dimmed(),
        }
    }
}

#[derive(Clone)]
pub struct EditorStyle {
    pub prompt: Style,
    pub prompt_prefix: Style,
    pub hint: Style,
    pub editor_command: Style,
    pub success: Style,
    pub error: Style,
    pub error_hint: Style,
}

impl Default for EditorStyle {
    fn default() -> Self {
        Self::mocha()
    }
}

impl EditorStyle {
    pub fn mocha() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<205, 214, 244>>(),
            prompt_prefix: Style::new().fg::<CustomColor<137, 180, 250>>(),
            hint: Style::new().fg::<CustomColor<127, 132, 156>>(),
            editor_command: Style::new().fg::<CustomColor<249, 226, 175>>().italic(),
            success: Style::new().fg::<CustomColor<166, 227, 161>>(),
            error: Style::new().fg::<CustomColor<243, 139, 168>>().bold(),
            error_hint: Style::new().fg::<CustomColor<147, 153, 178>>(),
        }
    }

    pub fn frappe() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<198, 208, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<140, 170, 238>>(),
            hint: Style::new().fg::<CustomColor<131, 139, 167>>(),
            editor_command: Style::new().fg::<CustomColor<229, 200, 144>>().italic(),
            success: Style::new().fg::<CustomColor<166, 209, 137>>(),
            error: Style::new().fg::<CustomColor<231, 130, 132>>().bold(),
            error_hint: Style::new().fg::<CustomColor<165, 173, 206>>(),
        }
    }

    pub fn macchiato() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<202, 211, 245>>(),
            prompt_prefix: Style::new().fg::<CustomColor<138, 173, 244>>(),
            hint: Style::new().fg::<CustomColor<128, 135, 162>>(),
            editor_command: Style::new().fg::<CustomColor<238, 212, 159>>().italic(),
            success: Style::new().fg::<CustomColor<166, 218, 149>>(),
            error: Style::new().fg::<CustomColor<237, 135, 150>>().bold(),
            error_hint: Style::new().fg::<CustomColor<153, 160, 187>>(),
        }
    }

    pub fn latte() -> Self {
        Self {
            prompt: Style::new().fg::<CustomColor<76, 79, 105>>(),
            prompt_prefix: Style::new().fg::<CustomColor<30, 102, 245>>(),
            hint: Style::new().fg::<CustomColor<156, 160, 176>>(),
            editor_command: Style::new().fg::<CustomColor<223, 142, 29>>().italic(),
            success: Style::new().fg::<CustomColor<64, 160, 43>>(),
            error: Style::new().fg::<CustomColor<210, 15, 57>>().bold(),
            error_hint: Style::new().fg::<CustomColor<124, 127, 147>>(),
        }
    }

    pub fn minimal() -> Self {
        Self {
            prompt: Style::new(),
            prompt_prefix: Style::new().bold(),
            hint: Style::new().dimmed(),
            editor_command: Style::new().italic(),
            success: Style::new(),
            error: Style::new().bold(),
            error_hint: Style::new().dimmed(),
        }
    }
}

pub fn color<const R: u8, const G: u8, const B: u8>() -> Style {
    Style::new().fg::<CustomColor<R, G, B>>()
}
