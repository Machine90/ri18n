use std::hash::Hash;

use crate::polyglot::common::{Naming, used_types::Types};

#[derive(Debug, Clone, Default)]
pub struct Field {
    pub is_public: bool,
    pub is_static: bool,
    pub is_final: bool,
    pub r#type: Types,
    pub name: Naming,
}

#[allow(unused)]
impl Field {
    pub fn public(tp: Types, name: &str) -> Self {
        Self {
            is_public: true,
            r#type: tp,
            name: Naming::new(name),
            ..Default::default()
        }
    }

    pub fn private(tp: Types, name: &str) -> Self {
        Self {
            r#type: tp,
            name: Naming::new(name),
            ..Default::default()
        }
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.name == other.name
    }
}

impl Eq for Field {}

impl Hash for Field {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r#type.hash(state);
        self.name.hash(state);
    }
}

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
