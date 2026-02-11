use {crate::option::AskOption, std::collections::HashSet};

pub struct MultiSelectField {
    pub prompt: String,
    pub options: Vec<AskOption>,
    pub default_selections: HashSet<usize>,
    pub page_size: usize,
    pub min_selections: Option<usize>,
    pub max_selections: Option<usize>,
    pub help: Option<String>,
}

impl MultiSelectField {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            options: Vec::new(),
            default_selections: HashSet::new(),
            page_size: 10,
            min_selections: None,
            max_selections: None,
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

    pub fn default_selections(mut self, indices: &[usize]) -> Self {
        self.default_selections = indices.iter().copied().collect();
        self
    }

    pub fn page_size(mut self, size: usize) -> Self {
        self.page_size = size.max(3);
        self
    }

    pub fn min(mut self, min: usize) -> Self {
        self.min_selections = Some(min);
        self
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max_selections = Some(max);
        self
    }

    pub fn help(mut self, h: impl Into<String>) -> Self {
        self.help = Some(h.into());
        self
    }
}
