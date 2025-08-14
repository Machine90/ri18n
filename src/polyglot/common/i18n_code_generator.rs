use std::collections::BTreeSet;

use crate::polyglot::common::{
    FormattedContent,
    data_struct::DataStruct,
    field::Field,
    i18n_method::Method,
    iso639_codes::{Iso639Entry, iso639_map},
    package::Package,
    statement::{Operation, RightHand, Statement},
    used_types::Types,
};

#[derive(Debug, Clone)]
pub struct InsertSupportLanguageListCall {
    pub language_iso639_code: String,
    pub iso639_entry: Iso639Entry,
}

impl PartialEq for InsertSupportLanguageListCall {
    fn eq(&self, other: &Self) -> bool {
        self.language_iso639_code == other.language_iso639_code
    }
}

impl Eq for InsertSupportLanguageListCall {}

impl PartialOrd for InsertSupportLanguageListCall {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.language_iso639_code
            .partial_cmp(&other.language_iso639_code)
    }
}

impl Ord for InsertSupportLanguageListCall {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.language_iso639_code.cmp(&other.language_iso639_code)
    }
}

#[derive(Debug, Clone)]
pub struct InsertI18nMapCall {
    pub language_iso639_code: String,
    pub i18n_key: String,

    pub i18n_content: FormattedContent,
}

#[derive(Debug, Clone)]
pub struct I18nCodeGenerator {
    pub package: Option<Package>,

    pub name: String,
    pub struct_support_language_list: DataStruct,
    pub fn_build_i18n_key: Method,
    pub fn_get_by_locale: Method,
    pub fn_get_by_locale_with_args: Method,
    pub const_support_languages: BTreeSet<Statement>,
    pub stat_build_support_language_list: Statement,
    pub call_insert_support_language_list: BTreeSet<InsertSupportLanguageListCall>,
    pub const_i18n_keys: BTreeSet<Statement>,
    pub stat_build_i18n_map: Statement,
    pub call_insert_i18n_map: Vec<InsertI18nMapCall>,
}

impl I18nCodeGenerator {
    pub fn insert_i18n_entry(
        &mut self,
        language_iso639_code: &str,
        key: &str,
        value: &str,
    ) -> &mut Self {
        let iso639_entry = iso639_map()
            .get(language_iso639_code)
            .expect(&format!(
                "Invalid input language iso639 code {language_iso639_code}"
            ))
            .clone();
        let language = InsertSupportLanguageListCall {
            language_iso639_code: language_iso639_code.to_string(),
            iso639_entry,
        };
        self.call_insert_support_language_list.insert(language);
        let i18n_entry = InsertI18nMapCall {
            language_iso639_code: language_iso639_code.to_string(),
            i18n_key: key.to_string(),
            i18n_content: FormattedContent {
                i18n_content: value.to_string(),
            },
        };
        self.call_insert_i18n_map.push(i18n_entry);

        let const_i18n_key = Statement {
            left_hand: Field {
                is_public: true,
                is_static: true,
                is_final: true,
                r#type: Types::String,
                name: key.replace(".", "_").to_uppercase(),
            },
            op: Operation::Assign,
            right_hand: RightHand::Value(format!("\"{key}\"")),
        };
        self.const_i18n_keys.insert(const_i18n_key);

        let const_support_lang = Statement {
            left_hand: Field {
                is_public: true,
                is_static: true,
                is_final: true,
                r#type: Types::String,
                name: language_iso639_code.replace("\\.", "_").to_uppercase(),
            },
            op: Operation::Assign,
            right_hand: RightHand::Value(format!("\"{language_iso639_code}\"")),
        };
        self.const_support_languages.insert(const_support_lang);
        self
    }
}
