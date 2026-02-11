pub struct ConfirmField {
    pub prompt: String,
    pub default: bool,
    pub yes_text: String,
    pub no_text: String,
}

impl ConfirmField {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            default: true,
            yes_text: "yes".into(),
            no_text: "no".into(),
        }
    }

    pub fn default(mut self, d: bool) -> Self {
        self.default = d;
        self
    }

    pub fn yes_text(mut self, t: impl Into<String>) -> Self {
        self.yes_text = t.into();
        self
    }

    pub fn no_text(mut self, t: impl Into<String>) -> Self {
        self.no_text = t.into();
        self
    }
}
