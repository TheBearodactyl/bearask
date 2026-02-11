pub struct SortField {
    pub prompt: String,
    pub items: Vec<String>,
    pub page_size: usize,
    pub help: Option<String>,
}

impl SortField {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            items: Vec::new(),
            page_size: 10,
            help: None,
        }
    }

    pub fn items(mut self, items: Vec<impl Into<String>>) -> Self {
        self.items = items.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn item(mut self, item: impl Into<String>) -> Self {
        self.items.push(item.into());
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
