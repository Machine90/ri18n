use std::collections::BTreeSet;

use crate::polyglot::common::{field::Field, statement::Statement};

#[derive(Debug, Default, Clone)]
pub struct DataStruct {
    pub is_static: bool,
    pub name: String,
    pub fields: BTreeSet<Field>,
    pub statements: BTreeSet<Statement>,
    pub with_constructor: bool,
    pub base_ident: u8,
}
