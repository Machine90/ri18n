use crate::polyglot::common::used_types::Types;

#[derive(Debug, Clone, Default)]
pub struct Field {
    pub is_public: bool,
    pub is_static: bool,
    pub r#type: Types,
    pub name: String,
}

impl Field {
    pub fn new(is_public: bool, is_static: bool, tp: Types, name: &str) -> Self {
        Self {
            is_public,
            is_static,
            r#type: tp,
            name: name.to_string(),
        }
    }

    pub fn public(tp: Types, name: &str) -> Self {
        Self::new(true, false, tp, name)
    }

    pub fn private(tp: Types, name: &str) -> Self {
        Self::new(false, false, tp, name)
    }
}
