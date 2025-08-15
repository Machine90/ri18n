use crate::polyglot::common::field::Field;

#[allow(unused)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum Types {
    #[default]
    Void,
    String,
    ListAny,
    List(Box<Types>),
    HashMap(Box<Types>, Box<Types>),
    Custom(&'static str),
}

#[derive(Debug, Clone)]
pub struct Construct {
    pub tp: Types,
    pub args: Vec<Field>,
}
