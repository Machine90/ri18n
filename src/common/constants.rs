pub const METHOD_NAME_TRANSLATE: &'static str = "translate";
/// Distinct from [`METHOD_NAME_TRANSLATE`] so languages without overloading
/// (and Rust macros) do not collide with the plain lookup API.
pub const METHOD_NAME_TRANSLATE_WITH_ARGS: &'static str = "translate_with_args";
pub const ARG_NAME_I18N_KEY: &'static str = "translation_key";
pub const ARG_NAME_ISO639: &'static str = "iso639_code";
pub const ARG_NAME_ARGS: &'static str = "args";
pub const STRUCT_NAME_SUPPORT_LANGUAGE: &'static str = "SupportLanguage";
pub const FIELD_NAME_CN_NAME: &str = "cn_name";
pub const FIELD_NAME_EN_NAME: &str = "en_name";
pub const FIELD_NAME_NATIVE_NAME: &str = "native_name";
pub const INSTANCE_NAME_LANGUAGE_LIST: &str = "language_list";
