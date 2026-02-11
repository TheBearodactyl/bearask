use owo_colors::Style;

/// an option in a (Multi)Select prompt
#[derive(Clone, Debug)]
pub struct AskOption {
    /// the name of the option
    pub name: String,
    /// the description of the option
    pub description: String,
    /// styling settings for displaying the option name
    pub name_style: Style,
    /// styling settings for displaying the option description
    pub description_style: Style,
}

impl AskOption {
    /// make a new option
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the option
    /// * `description` - the description of the option
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            name_style: crate::style::MultiSelectStyle::default().option_name,
            description: description.into(),
            description_style: crate::style::MultiSelectStyle::default().option_description,
        }
    }

    /// make an option with an empty description
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the option
    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            name_style: crate::style::MultiSelectStyle::default().option_name,
            description: String::new(),
            description_style: crate::style::MultiSelectStyle::default().option_description,
        }
    }

    /// set the styling of the option name
    ///
    /// # Arguments
    ///
    /// * `style` - the styling settings for the option
    pub fn name_style(mut self, style: Style) -> Self {
        self.name_style = style;
        self
    }

    /// set the styling of the option description
    ///
    /// # Arguments
    ///
    /// * `style` - the styling settings for the option description
    pub fn desc_style(mut self, style: Style) -> Self {
        self.description_style = style;
        self
    }
}
