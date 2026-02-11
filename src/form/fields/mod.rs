pub mod action;
pub mod confirm;
pub mod kind;
pub mod multiselect;
pub mod number;
pub mod password;
pub mod select;
pub mod sort;
pub mod text;

pub trait IntoFormField {
    fn into_field_kind(self) -> FieldKind;
}

impl IntoFormField for TextField {
    fn into_field_kind(self) -> FieldKind {
        let input = self.default.clone().unwrap_or_default();
        let cursor_pos = input.len();
        FieldKind::Text {
            prompt: self.prompt,
            placeholder: self.placeholder,
            default: self.default,
            help: self.help,
            validation: self.validation,
            input,
            cursor_pos,
        }
    }
}

impl IntoFormField for ConfirmField {
    fn into_field_kind(self) -> FieldKind {
        FieldKind::Confirm {
            prompt: self.prompt,
            default: self.default,
            yes_text: self.yes_text,
            no_text: self.no_text,
            selected: self.default,
        }
    }
}

impl IntoFormField for SelectField {
    fn into_field_kind(self) -> FieldKind {
        FieldKind::Select {
            prompt: self.prompt,
            options: self.options,
            page_size: self.page_size,
            help: self.help,
            cursor: 0,
            scroll_offset: 0,
        }
    }
}

impl IntoFormField for MultiSelectField {
    fn into_field_kind(self) -> FieldKind {
        let selected = self.default_selections.clone();
        FieldKind::MultiSelect {
            prompt: self.prompt,
            options: self.options,
            page_size: self.page_size,
            min_selections: self.min_selections,
            max_selections: self.max_selections,
            help: self.help,
            cursor: 0,
            scroll_offset: 0,
            selected,
        }
    }
}

impl IntoFormField for IntField {
    fn into_field_kind(self) -> FieldKind {
        let input = self.default.map(|d| d.to_string()).unwrap_or_default();
        let cursor_pos = input.len();
        FieldKind::Int {
            prompt: self.prompt,
            default: self.default,
            min: self.min,
            max: self.max,
            step: self.step,
            help: self.help,
            validation: self.validation,
            input,
            cursor_pos,
        }
    }
}

impl IntoFormField for FloatField {
    fn into_field_kind(self) -> FieldKind {
        let input = self.default.map(|d| d.to_string()).unwrap_or_default();
        let cursor_pos = input.len();
        FieldKind::Float {
            prompt: self.prompt,
            default: self.default,
            min: self.min,
            max: self.max,
            step: self.step,
            help: self.help,
            validation: self.validation,
            input,
            cursor_pos,
        }
    }
}

impl IntoFormField for PasswordField {
    fn into_field_kind(self) -> FieldKind {
        FieldKind::Password {
            prompt: self.prompt,
            mask_char: self.mask_char,
            min_length: self.min_length,
            max_length: self.max_length,
            help: self.help,
            validation: self.validation,
            input: String::new(),
            cursor_pos: 0,
            revealed: false,
        }
    }
}

impl IntoFormField for SortField {
    fn into_field_kind(self) -> FieldKind {
        let items = self.items.clone();
        FieldKind::Sort {
            prompt: self.prompt,
            page_size: self.page_size,
            help: self.help,
            items,
            cursor: 0,
            grabbed: false,
            scroll_offset: 0,
        }
    }
}

pub use {
    action::*, confirm::*, kind::*, multiselect::*, number::*, password::*, select::*, sort::*,
    text::*,
};
