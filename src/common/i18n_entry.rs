use crate::polyglot::common::FormatContent;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct I18nEntry {
    pub(crate) key: String,
    pub(crate) value: FormatContent,
}
