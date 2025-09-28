use std::collections::{BTreeMap, BTreeSet};

use crate::{
    common::i18n_entry::I18nEntry,
    polyglot::common::{
        FormatContent, Naming,
        i18n_method::{Method, MethodCall},
    },
};

#[derive(Debug, Default)]
pub struct I18nSwitch {
    pub arg_name_i18n_key: Naming,
    pub entries: BTreeSet<I18nEntry>,
    pub base_ident: u8,
}

#[derive(Debug, Default)]
pub struct LanguageMatcher {
    pub arg_name_iso639: Naming,
    pub arg_name_i18n_key: Naming,
    pub language_to_fn_match_entries: BTreeMap<String, MethodCall>,
    pub base_ident: u8,
}

impl I18nSwitch {
    pub(crate) fn insert(&mut self, i18n_key: &str, i18n_value: &str) -> &mut Self {
        self.entries.insert(I18nEntry {
            key: i18n_key.to_string(),
            value: FormatContent {
                i18n_content: i18n_value.to_string(),
            },
        });
        self
    }
}

impl LanguageMatcher {
    pub(crate) fn insert(
        &mut self,
        language_iso639_code: &str,
        fn_match_entries: &Method,
    ) -> &mut Self {
        if !self
            .language_to_fn_match_entries
            .contains_key(language_iso639_code)
        {
            let method_call = MethodCall::from(fn_match_entries);
            self.language_to_fn_match_entries
                .insert(language_iso639_code.to_string(), method_call);
        }
        self
    }
}
