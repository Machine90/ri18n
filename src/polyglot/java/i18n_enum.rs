use std::collections::HashSet;

use crate::polyglot::{
    ident_lines::IdentLines, iso639_codes::iso639_set, java::constants::ENUM_NAME_ISO_639_CODE,
};

#[derive(Debug, Default)]
pub(crate) struct I18nEnum {
    pub(crate) name: Option<String>,
    pub(crate) language_iso639_codes: HashSet<String>,
    pub(crate) base_ident: u8,
}

impl ToString for I18nEnum {
    fn to_string(&self) -> String {
        if self.language_iso639_codes.is_empty() {
            return format!("");
        }
        let enum_name = self
            .name
            .clone()
            .unwrap_or(ENUM_NAME_ISO_639_CODE.to_string());
        let mut lines = IdentLines::new(self.base_ident);
        lines.push(format!("public static enum {enum_name} {{"));
        for iso639_code in self.language_iso639_codes.iter() {
            let name = iso639_code.to_uppercase();
            let value = iso639_code.to_lowercase();
            if !iso639_set().contains_key(value.as_str()) {
                panic!("Invalid language code");
            }
            lines.push(format!("\t{name}(\"{value}\"),"));
        }
        lines.push("\t;".to_string());
        lines.next_line_if_true(true);
        lines.push("\tprivate String code;".to_string());
        lines.next_line_if_true(true);
        lines
            .push(format!("\t{enum_name}(String code) {{"))
            .push("\t\tthis.code = code;".to_string())
            .push("\t}".to_string());
        lines.push("}".to_string()).generate()
    }
}
