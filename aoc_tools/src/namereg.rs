use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
pub struct NameRegistry {
    names: HashMap<String, usize>,
}

impl NameRegistry {
    pub fn new() -> Self {
        Self {
            names: HashMap::new(),
        }
    }

    pub fn add_or_lookup<S: AsRef<str>>(&mut self, name: S) -> usize {
        let next_id = self.names.len();
        *self.names.entry(name.as_ref().to_owned()).or_insert(next_id)
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }
}

impl From<NameRegistry> for Vec<String> {
    fn from(value: NameRegistry) -> Self {
        value.names
            .into_iter()
            .sorted_by_key(|(_, i)| *i)
            .map(|(s, _)| s)
            .collect()
    }
}

