use std::hash::Hash;

use crate::polyglot::common::FormatContent;

#[derive(Debug, Clone)]
pub(crate) struct I18nEntry {
    pub(crate) key: String,
    pub(crate) value: FormatContent,
}

impl Hash for I18nEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl Eq for I18nEntry {}

impl PartialEq for I18nEntry {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl PartialOrd for I18nEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl Ord for I18nEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}
