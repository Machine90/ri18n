#![allow(unused)]

pub mod data_struct;
pub mod field;
pub mod i18n_code_generator;
pub mod i18n_method;
pub mod i18n_switch;
pub mod module;
pub mod package;
pub mod statement;
pub mod used_types;

use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref REGEX: Regex =
        Regex::new(r"\{(string|s|str)\}|\{c\}|\{char\}|\{u32\}|\{u64\}|\{u128\}|\{i32\}|\{i64\}|\{u8\}|\{u16\}|\{usize\}|\{f32\}|\{f64\}|\{bool\}|\{hex\}|\{exp\}")
            .expect("Invalid regex");
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    String,
    Int32,
    Float,
    Char,
    Bool,
    Hex,
    Exponent,
}

/// The content of i18n entry, e.g. "User name is {string} age is {u32} weight is {f32}"
/// can be replaced with difference programming language format,
/// for example java: "User name is %s age is %d  weight is %f".
/// ## Arg type:
/// * {string} | {s} | {str} => Format::String
/// * {u32} | {u64} => Format::Int32
/// * {f32} | {f64} => Format::Float
/// * {c} | {char} => Format::Char
/// * {bool} => Format::Bool
/// * {hex} => Format::Hex
/// * {exp} => Format::Exponent
#[derive(Debug, Clone)]
pub struct FormatContent {
    pub i18n_content: String,
}

impl FormatContent {
    pub fn replace(&self, matcher: fn(Format) -> &'static str) -> String {
        REGEX
            .replace_all(&self.i18n_content, |caps: &regex::Captures<'_>| {
                match caps.get(0).unwrap().as_str() {
                    "{string}" | "{s}" | "{str}" => matcher(Format::String),
                    "{u32}" | "{u64}" | "{i32}" | "{i64}" | "{u8}" | "{u16}" | "{u128}"
                    | "{usize}" => matcher(Format::Int32),
                    "{f32}" | "{f64}" => matcher(Format::Float),
                    "{c}" | "{char}" => matcher(Format::Char),
                    "{bool}" => matcher(Format::Bool),
                    "{hex}" => matcher(Format::Hex),
                    "{exp}" => matcher(Format::Exponent),
                    _ => "",
                }
                .to_string()
            })
            .to_string()
    }
}

pub fn java_matcher(fmt: Format) -> &'static str {
    match fmt {
        Format::String => "%s",
        Format::Int32 => "%d",
        Format::Float => "%f",
        Format::Char => "%c",
        Format::Bool => "%b",
        Format::Hex => "%x",
        Format::Exponent => "%e",
    }
}

pub fn rust_matcher(fmt: Format) -> &'static str {
    match fmt {
        Format::String
        | Format::Int32
        | Format::Float
        | Format::Char
        | Format::Bool
        | Format::Hex
        | Format::Exponent => "{}",
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Naming {
    pub name: String,
}

impl Naming {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn replace(&self, convention: fn(&str) -> String) -> String {
        convention(&self.name)
    }
}

pub fn to_camelcase(name: &str) -> String {
    name.to_case(Case::Camel)
}

pub fn to_snake_case(name: &str) -> String {
    name.to_case(Case::Snake)
}

pub fn to_ada(name: &str) -> String {
    name.to_case(Case::Ada)
}

pub fn to_pascal(name: &str) -> String {
    name.to_case(Case::Pascal)
}

pub fn to_constant(name: &str) -> String {
    name.to_case(Case::Constant)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_convert() {
        let expected = "User name is %s age is %d weight is %f";
        let fmt = FormatContent {
            i18n_content: "User name is {str} age is {u32} weight is {f32}".to_string(),
        };
        let c = fmt.replace(java_matcher);
        assert_eq!(expected, c);
    }

    #[test]
    fn test_case_convert() {
        let camelcase = Naming {
            name: "this_is_snake_case".to_string(),
        }
        .replace(to_camelcase);
        assert_eq!(camelcase, "thisIsSnakeCase");

        let snake_case = Naming {
            name: "thisIs_Camel_case".to_string(),
        }
        .replace(to_snake_case);
        assert_eq!(snake_case, "this_is_camel_case");

        let constant_case = Naming {
            name: "thisIs_Camel_case".to_string(),
        }
        .replace(to_constant);
        assert_eq!(constant_case, "THIS_IS_CAMEL_CASE");
    }
}
