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
    /// Placeholder token allowlist. Unknown `{...}` tokens are left unchanged.
    pub static ref REGEX: Regex = Regex::new(
        r"\{(string|s|str|text|c|char|u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize|int|integer|long|short|byte|f32|f64|float|double|bool|boolean|hex|exp)\}"
    )
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
/// * {string} | {s} | {str} | {text} => Format::String
/// * {u8} | {u16} | {u32} | {u64} | {u128} | {usize}
///   | {i8} | {i16} | {i32} | {i64} | {i128} | {isize}
///   | {int} | {integer} | {long} | {short} | {byte} => Format::Int32
/// * {f32} | {f64} | {float} | {double} => Format::Float
/// * {c} | {char} => Format::Char
/// * {bool} | {boolean} => Format::Bool
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
                let token = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                match token {
                    "string" | "s" | "str" | "text" => matcher(Format::String),
                    "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32"
                    | "i64" | "i128" | "isize" | "int" | "integer" | "long" | "short" | "byte" => {
                        matcher(Format::Int32)
                    }
                    "f32" | "f64" | "float" | "double" => matcher(Format::Float),
                    "c" | "char" => matcher(Format::Char),
                    "bool" | "boolean" => matcher(Format::Bool),
                    "hex" => matcher(Format::Hex),
                    "exp" => matcher(Format::Exponent),
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
