use crate::polyglot::common::field::Field;

#[derive(Debug, Default, Clone)]
pub enum Types {
    #[default]
    Void,
    String,
    List(Box<Types>),
    HashMap(Box<Types>, Box<Types>),
    Custom(&'static str),
}

#[derive(Debug, Clone)]
pub struct Construct {
    pub tp: Types,
    pub args: Vec<Field>,
}
