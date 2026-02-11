use owo_colors::Style;

#[derive(Clone, Debug)]
pub struct AskOption {
    pub name: String,
    pub description: String,
    pub name_style: Style,
    pub description_style: Style,
}

impl AskOption {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            name_style: crate::style::MultiSelectStyle::default().option_name,
            description: description.into(),
            description_style: crate::style::MultiSelectStyle::default().option_description,
        }
    }

    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            name_style: crate::style::MultiSelectStyle::default().option_name,
            description: String::new(),
            description_style: crate::style::MultiSelectStyle::default().option_description,
        }
    }

    pub fn name_style(mut self, style: Style) -> Self {
        self.name_style = style;
        self
    }

    pub fn desc_style(mut self, style: Style) -> Self {
        self.description_style = style;
        self
    }
}
