pub struct TextField {
    pub prompt: String,
    pub placeholder: Option<String>,
    pub default: Option<String>,
    pub help: Option<String>,
    pub validation: Option<fn(&str) -> Result<(), String>>,
}

impl TextField {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            placeholder: None,
            default: None,
            help: None,
            validation: None,
        }
    }

    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = Some(p.into());
        self
    }

    pub fn default(mut self, d: impl Into<String>) -> Self {
        self.default = Some(d.into());
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
