use crate::form::value::FormValue;

#[derive(Clone, Debug)]
pub struct FormResult {
    pub entries: Vec<(String, FormValue)>,
}

impl FormResult {
    pub fn get(&self, key: &str) -> Option<&FormValue> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &FormValue)> {
        self.entries.iter().map(|(k, v)| (k.as_str(), v))
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn into_entries(self) -> Vec<(String, FormValue)> {
        self.entries
    }
}

impl std::ops::Index<&str> for FormResult {
    type Output = FormValue;

    fn index(&self, key: &str) -> &Self::Output {
        self.get(key)
            .unwrap_or_else(|| panic!("no form field with key '{}'", key))
    }
}
