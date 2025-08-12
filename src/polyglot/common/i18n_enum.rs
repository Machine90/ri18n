use std::collections::HashSet;

#[derive(Debug, Default)]
pub(crate) struct I18nEnum {
    pub(crate) name: String,
    pub(crate) language_iso639_codes: HashSet<String>,
    pub(crate) base_ident: u8,
}
