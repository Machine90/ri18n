use std::collections::HashMap;

use crate::polyglot::{ident_lines::IdentLines, java::constants::*};

#[derive(Debug)]
pub(crate) struct I18nEntry {
    pub language_iso639_code: String,
    pub value: String,
}

#[derive(Debug, Default)]
pub(crate) struct I18nMapper {
    pub(crate) i18n_entries: HashMap<String, Vec<I18nEntry>>,
    pub(crate) base_ident: u8,
}

impl I18nMapper {
    pub fn add_entry(&mut self, lang: &str, key: &str, value: &str) {
        let entry = I18nEntry {
            language_iso639_code: lang.to_string(),
            value: value.to_string(),
        };
        self.i18n_entries
            .entry(key.to_string())
            .or_insert(Vec::<I18nEntry>::new())
            .push(entry);
    }
}

impl ToString for I18nMapper {
    fn to_string(&self) -> String {
        const MAPPER_NAME: &str = "languageMapper";
        let mut lines = IdentLines::new(self.base_ident);
        lines.push(format!(
            "{JAVA_STATIC} {JAVA_FINAL} java.util.Map<{JAVA_STRING}, {JAVA_STRING}> {MAPPER_NAME} = new java.util.HashMap<>();"
        ));
        lines.push(format!("{JAVA_STATIC} {{"));
        for (i18n_key, entries) in self.i18n_entries.iter() {
            let field = i18n_key.replace(".", "_").to_uppercase();
            for I18nEntry {
                language_iso639_code,
                value,
            } in entries
            {
                let lang = language_iso639_code.to_uppercase();
                let statement = format!(
                    "\t{MAPPER_NAME}.put({METHOD_NAME_BUILD_I18N_KEY}({ENUM_NAME_ISO_639_CODE}.{lang}, {field}), \"{value}\");"
                );
                lines.push(statement);
            }
        }
        lines.push(format!("}}"));

        lines.generate()
    }
}
