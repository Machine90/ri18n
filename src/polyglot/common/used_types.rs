use crate::polyglot::common::field::Field;

#[allow(unused)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum Types {
    #[default]
    Void,
    String,
    RefString,
    /// Variadic formatting arguments for `translate_with_args`.
    ///
    /// Backend contract:
    /// - Java: render as `Object...`
    /// - Rust: cannot appear in a normal `fn` signature; the Rust backend must
    ///   implement `TranslateWithArgs` via the `translate_with_args!`
    ///   macro strategy (when formatx is enabled) instead of a varargs method.
    VarArgs,
    List(Box<Types>),
    HashMap(Box<Types>, Box<Types>),
    Option(Box<Types>),
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Construct {
    pub tp: Types,
    pub args: Vec<Field>,
}
