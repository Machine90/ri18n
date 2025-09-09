use std::collections::{BTreeMap, BTreeSet};

use crate::{
    common::i18n_entry::I18nEntry,
    polyglot::common::{FormatContent, Naming},
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
    pub language_to_entries: BTreeMap<String, I18nSwitch>,
    pub base_ident: u8,
}

impl LanguageMatcher {
    pub(crate) fn insert(
        &mut self,
        iso639_code: &str,
        i18n_key: &str,
        i18n_value: &str,
    ) -> &mut Self {
        self.language_to_entries
            .entry(iso639_code.to_string())
            .or_insert(I18nSwitch {
                arg_name_i18n_key: self.arg_name_i18n_key.clone(),
                base_ident: self.base_ident + 1,
                ..Default::default()
            })
            .entries
            .insert(I18nEntry {
                key: i18n_key.to_string(),
                value: FormatContent {
                    i18n_content: i18n_value.to_string(),
                },
            });
        self
    }
}
