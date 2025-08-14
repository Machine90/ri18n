pub mod code_lines;
pub mod constants;
pub mod data_struct;
pub mod field;
pub mod i18n_code_generator;
pub mod i18n_method;
pub mod iso639_codes;
pub mod package;
pub mod statement;
pub mod used_types;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref REGEX: Regex =
        Regex::new(r"\{(string|s|str)\}|\{c\}|\{char\}|\{u32\}|\{u64\}|\{f32\}|\{f64\}|\{bool\}|\{hex\}|\{exp\}")
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
pub struct FormattedContent {
    pub i18n_content: String,
}

impl FormattedContent {
    pub fn replace(&self, matcher: fn(Format) -> &'static str) -> String {
        REGEX
            .replace_all(&self.i18n_content, |caps: &regex::Captures<'_>| {
                match caps.get(0).unwrap().as_str() {
                    "{string}" | "{s}" | "{str}" => matcher(Format::String),
                    "{u32}" | "{u64}" => matcher(Format::Int32),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let expected = "User name is %s age is %d weight is %f";
        let fmt = FormattedContent {
            i18n_content: "User name is {str} age is {u32} weight is {f32}".to_string(),
        };
        let c = fmt.replace(java_matcher);
        assert_eq!(expected, c);
    }
}
