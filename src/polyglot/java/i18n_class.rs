use std::collections::BTreeSet;

use crate::polyglot::{
    ident_lines::IdentLines,
    java::{i18n_enum::I18nEnum, i18n_mapper_initializer::I18nMapper, i18n_method::Method},
};

use crate::polyglot::java::constants::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Constants {
    i18n_key: String,
}

impl ToString for Constants {
    fn to_string(&self) -> String {
        let field = self.i18n_key.replace(".", "_").to_uppercase();
        format!(
            "{JAVA_PUBLIC} {JAVA_STATIC} {JAVA_FINAL} {JAVA_STRING} {field} = \"{}\";",
            self.i18n_key
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Import {
    pub package: String,
}

impl ToString for Import {
    fn to_string(&self) -> String {
        format!("{JAVA_IMPORT} {};", self.package)
    }
}

#[derive(Debug, Default)]
pub struct I18nClass {
    pub package: Option<String>,
    pub class_name: Option<String>,
    pub imports: BTreeSet<Import>,
    pub constants: BTreeSet<Constants>,
    pub i18n_enum: I18nEnum,

    pub i18n_mapper: I18nMapper,
    pub methods: Vec<Method>,
    pub base_ident: u8,
}

impl I18nClass {
    pub fn get_class_name(&self) -> String {
        self.class_name.clone().unwrap_or("I18nUtil".to_string())
    }

    pub fn add_const(&mut self, constant: Constants) -> &mut Self {
        self.constants.insert(constant);
        self
    }

    pub fn add_i18n_entry(&mut self, lang: &str, key: &str, value: &str) -> &mut Self {
        self.add_const(Constants {
            i18n_key: key.to_string(),
        });
        self.i18n_enum
            .language_iso639_codes
            .insert(lang.to_string());
        self.i18n_mapper.add_entry(lang, key, value);
        self
    }
}

impl ToString for I18nClass {
    fn to_string(&self) -> String {
        let mut lines = IdentLines::new(self.base_ident);
        if let Some(package) = self.package.as_ref() {
            lines
                .push(format!("{JAVA_PKG} {package};"))
                .push("\n".to_string());
        }

        for import in self.imports.iter() {
            lines.push(import.to_string());
        }
        lines.next_line_if_true(!self.imports.is_empty());

        let class_name = self.get_class_name();
        lines.push(format!("{JAVA_PUBLIC} class {class_name} {{"));

        // write methods
        for method in self.methods.iter() {
            lines.push(method.to_string());
        }
        lines.next_line_if_true(!self.methods.is_empty());

        // write inner class or enum
        lines.push(self.i18n_enum.to_string());
        lines.next_line_if_true(true);

        // write constants code
        for constant in self.constants.iter() {
            lines.push(format!("\t{}", constant.to_string()));
        }
        lines.next_line_if_true(!self.constants.is_empty());

        // write static setup
        lines.push(self.i18n_mapper.to_string());
        lines.next_line_if_true(true);

        lines.push(format!("}}")).generate()
    }
}
