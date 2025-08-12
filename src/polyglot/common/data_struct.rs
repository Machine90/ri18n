use crate::polyglot::common::field::Field;

#[derive(Debug, Default, Clone)]
pub struct DataStruct {
    pub name: String,
    pub fields: Vec<Field>,
    pub base_ident: u8,
}
