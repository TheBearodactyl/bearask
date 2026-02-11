pub struct PasswordField {
    pub prompt: String,
    pub mask_char: char,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub help: Option<String>,
    pub validation: Option<fn(&str) -> Result<(), String>>,
}

impl PasswordField {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            mask_char: '●',
            min_length: None,
            max_length: None,
            help: None,
            validation: None,
        }
    }

    pub fn mask_char(mut self, c: char) -> Self {
        self.mask_char = c;
        self
    }

    pub fn min_length(mut self, m: usize) -> Self {
        self.min_length = Some(m);
        self
    }

    pub fn max_length(mut self, m: usize) -> Self {
        self.max_length = Some(m);
        self
    }

    pub fn help(mut self, h: impl Into<String>) -> Self {
        self.help = Some(h.into());
        self
    }

    pub fn validation(mut self, v: fn(&str) -> Result<(), String>) -> Self {
        self.validation = Some(v);
        self
    }
}
