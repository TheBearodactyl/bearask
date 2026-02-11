pub struct IntField {
    pub prompt: String,
    pub default: Option<i64>,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub step: i64,
    pub help: Option<String>,
    pub validation: Option<fn(i64) -> Result<(), String>>,
}

impl IntField {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            default: None,
            min: None,
            max: None,
            step: 1,
            help: None,
            validation: None,
        }
    }

    pub fn default(mut self, d: i64) -> Self {
        self.default = Some(d);
        self
    }

    pub fn min(mut self, m: i64) -> Self {
        self.min = Some(m);
        self
    }

    pub fn max(mut self, m: i64) -> Self {
        self.max = Some(m);
        self
    }

    pub fn step(mut self, s: i64) -> Self {
        self.step = s;
        self
    }

    pub fn help(mut self, h: impl Into<String>) -> Self {
        self.help = Some(h.into());
        self
    }

    pub fn validation(mut self, v: fn(i64) -> Result<(), String>) -> Self {
        self.validation = Some(v);
        self
    }
}

pub struct FloatField {
    pub prompt: String,
    pub default: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: f64,
    pub help: Option<String>,
    pub validation: Option<fn(f64) -> Result<(), String>>,
}

impl FloatField {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            default: None,
            min: None,
            max: None,
            step: 1.0,
            help: None,
            validation: None,
        }
    }

    pub fn default(mut self, d: f64) -> Self {
        self.default = Some(d);
        self
    }

    pub fn min(mut self, m: f64) -> Self {
        self.min = Some(m);
        self
    }

    pub fn max(mut self, m: f64) -> Self {
        self.max = Some(m);
        self
    }

    pub fn step(mut self, s: f64) -> Self {
        self.step = s;
        self
    }

    pub fn help(mut self, h: impl Into<String>) -> Self {
        self.help = Some(h.into());
        self
    }

    pub fn validation(mut self, v: fn(f64) -> Result<(), String>) -> Self {
        self.validation = Some(v);
        self
    }
}
