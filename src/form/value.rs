#[derive(Clone, Debug)]
pub enum FormValue {
    Text(String),
    Bool(bool),
    Index(usize),
    Indices(Vec<usize>),
    Int(i64),
    Float(f64),
    Sorted(Vec<String>),
}

impl FormValue {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_index(&self) -> Option<usize> {
        match self {
            Self::Index(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_indices(&self) -> Option<&[usize]> {
        match self {
            Self::Indices(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_sorted(&self) -> Option<&[String]> {
        match self {
            Self::Sorted(v) => Some(v),
            _ => None,
        }
    }
}
