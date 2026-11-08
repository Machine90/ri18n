use std::collections::{BTreeMap, BTreeSet};

use crate::{
    common::iso639_codes::{Iso639Entry, iso639_map},
    polyglot::common::{
        Naming,
        data_struct::DataStruct,
        field::Field,
        i18n_method::Method,
        package::{Package, PackageAction},
        statement::{Operation, RightHand, Statement},
        used_types::Types,
    },
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

#[derive(Debug)]
pub struct I18nCodeGenerator {
    pub package: Option<PackageAction>,
    pub name: String,
    pub struct_support_language_list: DataStruct,
    pub stat_build_support_language_list: Statement,
    pub call_insert_support_language_list: BTreeSet<InsertSupportLanguageListCall>,
    pub const_support_languages: BTreeSet<Statement>,
    pub const_i18n_keys: BTreeSet<Statement>,
    pub fn_translate: Option<Method>,
    pub fn_translate_with_args: Option<Method>,
    pub fn_match_entries: BTreeMap<String, Method>,
    /// Per-locale key sets for cross-locale consistency checks.
    pub keys_by_locale: BTreeMap<String, BTreeSet<String>>,
}

impl I18nCodeGenerator {
    pub fn insert_i18n_entry(
        &mut self,
        language_iso639_code: &str,
        key: &str,
        value: &str,
        generate_constants_only: bool,
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
        self.keys_by_locale
            .entry(language_iso639_code.to_string())
            .or_default()
            .insert(key.to_string());

        if !generate_constants_only {
            let fn_match_entries = self
                .fn_match_entries
                .entry(language_iso639_code.to_string())
                .or_insert(Method::new_method_match_entries(
                    language_iso639_code,
                    None,
                    1,
                ));
            fn_match_entries.insert_match_entries(key, value);
            self.fn_translate
                .as_mut()
                .map(|v| v.insert_translate(language_iso639_code, &fn_match_entries));
        }

        let const_i18n_key = Statement {
            left_hand: Field {
                is_public: true,
                is_static: true,
                is_final: true,
                r#type: Types::RefString,
                name: Naming::new(key.replace(".", "_").to_uppercase()),
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
                r#type: Types::RefString,
                name: Naming::new(language_iso639_code.replace("\\.", "_").to_uppercase()),
            },
            op: Operation::Assign,
            right_hand: RightHand::Value(format!("\"{language_iso639_code}\"")),
        };
        self.const_support_languages.insert(const_support_lang);
        self
    }

    /// Warn when locales do not share the same key set.
    pub fn warn_locale_key_mismatches(&self) {
        if self.keys_by_locale.len() < 2 {
            return;
        }
        let mut union = BTreeSet::new();
        for keys in self.keys_by_locale.values() {
            union.extend(keys.iter().cloned());
        }
        for (locale, keys) in &self.keys_by_locale {
            let missing: Vec<&String> = union.difference(keys).collect();
            if !missing.is_empty() {
                tracing::warn!(
                    locale = %locale,
                    missing = ?missing,
                    "locale is missing translation keys present in other locales"
                );
            }
        }
    }
}
