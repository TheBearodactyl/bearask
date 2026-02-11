use crate::option::AskOption;

pub struct SelectField {
    pub prompt: String,
    pub options: Vec<AskOption>,
    pub page_size: usize,
    pub help: Option<String>,
}

impl SelectField {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            options: Vec::new(),
            page_size: 10,
            help: None,
        }
    }

    pub fn options(mut self, opts: Vec<AskOption>) -> Self {
        self.options = opts;
        self
    }

    pub fn option(mut self, opt: AskOption) -> Self {
        self.options.push(opt);
        self
    }

    pub fn option_name(mut self, name: impl Into<String>) -> Self {
        self.options.push(AskOption::with_name(name));
        self
    }

    pub fn page_size(mut self, size: usize) -> Self {
        self.page_size = size.max(3);
        self
    }

    pub fn help(mut self, h: impl Into<String>) -> Self {
        self.help = Some(h.into());
        self
    }
}
