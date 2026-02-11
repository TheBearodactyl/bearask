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

#[derive(Clone)]
pub struct FormStyle {
    pub title: Style,
    pub title_prefix_style: Style,
    pub title_prefix: Option<String>,
    pub section_title: Style,
    pub section_decoration: Style,
    pub prompt: Style,
    pub active_prompt: Style,
    pub active_marker: Style,
    pub active_input: Style,
    pub active_selected: Style,
    pub placeholder: Style,
    pub default_value: Style,
    pub hint: Style,
    pub option: Style,
    pub checkbox_on: Style,
    pub checkbox_off: Style,
    pub password_mask: Style,
    pub sort_grabbed: Style,
    pub completed_prefix: Style,
    pub completed_value: Style,
    pub pending_prefix: Style,
    pub pending_prompt: Style,
    pub error: Style,
    pub error_hint: Style,
    pub nav_hint: Style,
    pub progress: Style,
    pub summary_text: Style,
    pub summary_prefix_style: Style,
    pub summary_prefix: Option<String>,
}

impl Default for FormStyle {
    fn default() -> Self {
        Self::mocha()
    }
}

impl FormStyle {
    pub fn mocha() -> Self {
        Self {
            title: Style::new().fg::<CustomColor<205, 214, 244>>().bold(),
            title_prefix_style: Style::new().fg::<CustomColor<137, 180, 250>>(),
            title_prefix: Some("◆".into()),
            section_title: Style::new().fg::<CustomColor<186, 194, 222>>().bold(),
            section_decoration: Style::new().fg::<CustomColor<127, 132, 156>>(),

            prompt: Style::new().fg::<CustomColor<205, 214, 244>>(),
            active_prompt: Style::new().fg::<CustomColor<205, 214, 244>>().bold(),
            active_marker: Style::new().fg::<CustomColor<137, 180, 250>>().bold(),
            active_input: Style::new().fg::<CustomColor<166, 227, 161>>(),
            active_selected: Style::new().fg::<CustomColor<137, 180, 250>>().bold(),
            placeholder: Style::new().fg::<CustomColor<127, 132, 156>>().italic(),
            default_value: Style::new().fg::<CustomColor<186, 194, 222>>().italic(),
            hint: Style::new().fg::<CustomColor<127, 132, 156>>(),
            option: Style::new().fg::<CustomColor<186, 194, 222>>(),
            checkbox_on: Style::new().fg::<CustomColor<166, 227, 161>>(),
            checkbox_off: Style::new().fg::<CustomColor<127, 132, 156>>(),
            password_mask: Style::new().fg::<CustomColor<186, 194, 222>>(),
            sort_grabbed: Style::new().fg::<CustomColor<166, 227, 161>>().bold(),

            completed_prefix: Style::new().fg::<CustomColor<166, 227, 161>>(),
            completed_value: Style::new().fg::<CustomColor<166, 227, 161>>(),

            pending_prefix: Style::new().fg::<CustomColor<127, 132, 156>>(),
            pending_prompt: Style::new().fg::<CustomColor<127, 132, 156>>().dimmed(),

            error: Style::new().fg::<CustomColor<243, 139, 168>>().bold(),
            error_hint: Style::new().fg::<CustomColor<147, 153, 178>>(),
            nav_hint: Style::new().fg::<CustomColor<127, 132, 156>>(),
            progress: Style::new().fg::<CustomColor<186, 194, 222>>().dimmed(),

            summary_text: Style::new().fg::<CustomColor<166, 227, 161>>().bold(),
            summary_prefix_style: Style::new().fg::<CustomColor<166, 227, 161>>(),
            summary_prefix: Some("◆".into()),
        }
    }

    pub fn frappe() -> Self {
        Self {
            title: Style::new().fg::<CustomColor<198, 208, 245>>().bold(),
            title_prefix_style: Style::new().fg::<CustomColor<140, 170, 238>>(),
            title_prefix: Some("◆".into()),
            section_title: Style::new().fg::<CustomColor<181, 191, 226>>().bold(),
            section_decoration: Style::new().fg::<CustomColor<131, 139, 167>>(),

            prompt: Style::new().fg::<CustomColor<198, 208, 245>>(),
            active_prompt: Style::new().fg::<CustomColor<198, 208, 245>>().bold(),
            active_marker: Style::new().fg::<CustomColor<140, 170, 238>>().bold(),
            active_input: Style::new().fg::<CustomColor<166, 209, 137>>(),
            active_selected: Style::new().fg::<CustomColor<140, 170, 238>>().bold(),
            placeholder: Style::new().fg::<CustomColor<131, 139, 167>>().italic(),
            default_value: Style::new().fg::<CustomColor<181, 191, 226>>().italic(),
            hint: Style::new().fg::<CustomColor<131, 139, 167>>(),
            option: Style::new().fg::<CustomColor<181, 191, 226>>(),
            checkbox_on: Style::new().fg::<CustomColor<166, 209, 137>>(),
            checkbox_off: Style::new().fg::<CustomColor<131, 139, 167>>(),
            password_mask: Style::new().fg::<CustomColor<181, 191, 226>>(),
            sort_grabbed: Style::new().fg::<CustomColor<166, 209, 137>>().bold(),

            completed_prefix: Style::new().fg::<CustomColor<166, 209, 137>>(),
            completed_value: Style::new().fg::<CustomColor<166, 209, 137>>(),

            pending_prefix: Style::new().fg::<CustomColor<131, 139, 167>>(),
            pending_prompt: Style::new().fg::<CustomColor<131, 139, 167>>().dimmed(),

            error: Style::new().fg::<CustomColor<231, 130, 132>>().bold(),
            error_hint: Style::new().fg::<CustomColor<165, 173, 206>>(),
            nav_hint: Style::new().fg::<CustomColor<131, 139, 167>>(),
            progress: Style::new().fg::<CustomColor<181, 191, 226>>().dimmed(),

            summary_text: Style::new().fg::<CustomColor<166, 209, 137>>().bold(),
            summary_prefix_style: Style::new().fg::<CustomColor<166, 209, 137>>(),
            summary_prefix: Some("◆".into()),
        }
    }

    pub fn macchiato() -> Self {
        Self {
            title: Style::new().fg::<CustomColor<202, 211, 245>>().bold(),
            title_prefix_style: Style::new().fg::<CustomColor<138, 173, 244>>(),
            title_prefix: Some("◆".into()),
            section_title: Style::new().fg::<CustomColor<184, 192, 224>>().bold(),
            section_decoration: Style::new().fg::<CustomColor<128, 135, 162>>(),

            prompt: Style::new().fg::<CustomColor<202, 211, 245>>(),
            active_prompt: Style::new().fg::<CustomColor<202, 211, 245>>().bold(),
            active_marker: Style::new().fg::<CustomColor<138, 173, 244>>().bold(),
            active_input: Style::new().fg::<CustomColor<166, 218, 149>>(),
            active_selected: Style::new().fg::<CustomColor<138, 173, 244>>().bold(),
            placeholder: Style::new().fg::<CustomColor<128, 135, 162>>().italic(),
            default_value: Style::new().fg::<CustomColor<184, 192, 224>>().italic(),
            hint: Style::new().fg::<CustomColor<128, 135, 162>>(),
            option: Style::new().fg::<CustomColor<184, 192, 224>>(),
            checkbox_on: Style::new().fg::<CustomColor<166, 218, 149>>(),
            checkbox_off: Style::new().fg::<CustomColor<128, 135, 162>>(),
            password_mask: Style::new().fg::<CustomColor<184, 192, 224>>(),
            sort_grabbed: Style::new().fg::<CustomColor<166, 218, 149>>().bold(),

            completed_prefix: Style::new().fg::<CustomColor<166, 218, 149>>(),
            completed_value: Style::new().fg::<CustomColor<166, 218, 149>>(),

            pending_prefix: Style::new().fg::<CustomColor<128, 135, 162>>(),
            pending_prompt: Style::new().fg::<CustomColor<128, 135, 162>>().dimmed(),

            error: Style::new().fg::<CustomColor<237, 135, 150>>().bold(),
            error_hint: Style::new().fg::<CustomColor<153, 160, 187>>(),
            nav_hint: Style::new().fg::<CustomColor<128, 135, 162>>(),
            progress: Style::new().fg::<CustomColor<184, 192, 224>>().dimmed(),

            summary_text: Style::new().fg::<CustomColor<166, 218, 149>>().bold(),
            summary_prefix_style: Style::new().fg::<CustomColor<166, 218, 149>>(),
            summary_prefix: Some("◆".into()),
        }
    }

    pub fn latte() -> Self {
        Self {
            title: Style::new().fg::<CustomColor<76, 79, 105>>().bold(),
            title_prefix_style: Style::new().fg::<CustomColor<30, 102, 245>>(),
            title_prefix: Some("◆".into()),
            section_title: Style::new().fg::<CustomColor<92, 95, 119>>().bold(),
            section_decoration: Style::new().fg::<CustomColor<156, 160, 176>>(),

            prompt: Style::new().fg::<CustomColor<76, 79, 105>>(),
            active_prompt: Style::new().fg::<CustomColor<76, 79, 105>>().bold(),
            active_marker: Style::new().fg::<CustomColor<30, 102, 245>>().bold(),
            active_input: Style::new().fg::<CustomColor<64, 160, 43>>(),
            active_selected: Style::new().fg::<CustomColor<30, 102, 245>>().bold(),
            placeholder: Style::new().fg::<CustomColor<156, 160, 176>>().italic(),
            default_value: Style::new().fg::<CustomColor<92, 95, 119>>().italic(),
            hint: Style::new().fg::<CustomColor<156, 160, 176>>(),
            option: Style::new().fg::<CustomColor<92, 95, 119>>(),
            checkbox_on: Style::new().fg::<CustomColor<64, 160, 43>>(),
            checkbox_off: Style::new().fg::<CustomColor<156, 160, 176>>(),
            password_mask: Style::new().fg::<CustomColor<92, 95, 119>>(),
            sort_grabbed: Style::new().fg::<CustomColor<64, 160, 43>>().bold(),

            completed_prefix: Style::new().fg::<CustomColor<64, 160, 43>>(),
            completed_value: Style::new().fg::<CustomColor<64, 160, 43>>(),

            pending_prefix: Style::new().fg::<CustomColor<156, 160, 176>>(),
            pending_prompt: Style::new().fg::<CustomColor<156, 160, 176>>().dimmed(),

            error: Style::new().fg::<CustomColor<210, 15, 57>>().bold(),
            error_hint: Style::new().fg::<CustomColor<124, 127, 147>>(),
            nav_hint: Style::new().fg::<CustomColor<156, 160, 176>>(),
            progress: Style::new().fg::<CustomColor<92, 95, 119>>().dimmed(),

            summary_text: Style::new().fg::<CustomColor<64, 160, 43>>().bold(),
            summary_prefix_style: Style::new().fg::<CustomColor<64, 160, 43>>(),
            summary_prefix: Some("◆".into()),
        }
    }

    pub fn minimal() -> Self {
        Self {
            title: Style::new().bold(),
            title_prefix_style: Style::new().bold(),
            title_prefix: Some("---".into()),
            section_title: Style::new().bold(),
            section_decoration: Style::new().dimmed(),

            prompt: Style::new(),
            active_prompt: Style::new().bold(),
            active_marker: Style::new().bold(),
            active_input: Style::new(),
            active_selected: Style::new().bold(),
            placeholder: Style::new().dimmed().italic(),
            default_value: Style::new().dimmed(),
            hint: Style::new().dimmed(),
            option: Style::new(),
            checkbox_on: Style::new(),
            checkbox_off: Style::new().dimmed(),
            password_mask: Style::new().dimmed(),
            sort_grabbed: Style::new().bold(),

            completed_prefix: Style::new(),
            completed_value: Style::new(),

            pending_prefix: Style::new().dimmed(),
            pending_prompt: Style::new().dimmed(),

            error: Style::new().bold(),
            error_hint: Style::new().dimmed(),
            nav_hint: Style::new().dimmed(),
            progress: Style::new().dimmed(),

            summary_text: Style::new().bold(),
            summary_prefix_style: Style::new().bold(),
            summary_prefix: Some("---".into()),
        }
    }
}

pub fn color<const R: u8, const G: u8, const B: u8>() -> Style {
    Style::new().fg::<CustomColor<R, G, B>>()
}
