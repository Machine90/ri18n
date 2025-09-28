use std::collections::BTreeSet;

use crate::polyglot::common::{Naming, statement::Statement};

/// Some language support module such as Rust
#[derive(Debug, Clone, Default)]
pub struct Module {
    pub is_public: bool,
    pub name: String,
    pub statements: BTreeSet<Statement>,
    pub base_ident: u8,
}
