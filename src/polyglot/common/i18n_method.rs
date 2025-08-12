use crate::polyglot::common::{field::Field, used_types::Types};

#[derive(Debug, Default, Clone)]
pub enum I18nMethodType {
    BuildI18nKey {
        arg_name_iso639: String,
        arg_name_i18n_key: String,
    },
    GetByLocale {
        method_name_build_i18n_key: String,
        arg_name_iso639: String,
        arg_name_i18n_key: String,
        instance_name_language_map: String,
    },
    GetByLocaleWithArgs {
        method_name_build_i18n_key: String,
        arg_name_iso639: String,
        arg_name_i18n_key: String,
        instance_name_language_map: String,
    },
    #[default]
    Empty,
}

#[derive(Debug, Default, Clone)]
pub struct I18nMethodContent {
    pub tp: I18nMethodType,
    pub base_ident: u8,
}

#[derive(Debug, Default, Clone)]
pub struct Method {
    pub name: String,
    pub args: Vec<Field>,
    pub return_type: Types,
    pub is_public: bool,
    pub is_static: bool,
    pub code: I18nMethodContent,
    pub base_ident: u8,
}

impl Method {
    pub fn new_method_build_i18n_key(
        method_name: Option<&'static str>,
        arg_name_iso639: Option<&'static str>,
        arg_name_i18n_key: Option<&'static str>,
        base_ident: u8,
    ) -> Self {
        let method_name = method_name.unwrap_or("build_i18n_key");
        let arg_name_iso639 = arg_name_iso639.unwrap_or("iso639_code");
        let arg_name_i18n_key = arg_name_i18n_key.unwrap_or("i18n_key");
        Self {
            name: method_name.to_string(),
            args: vec![
                Field {
                    r#type: Types::String,
                    name: arg_name_iso639.to_string(),
                    ..Default::default()
                },
                Field {
                    r#type: Types::String,
                    name: arg_name_i18n_key.to_string(),
                    ..Default::default()
                },
            ],
            return_type: Types::String,
            is_public: true,
            is_static: true,
            code: I18nMethodContent {
                tp: I18nMethodType::BuildI18nKey {
                    arg_name_iso639: arg_name_iso639.to_string(),
                    arg_name_i18n_key: arg_name_i18n_key.to_string(),
                },
                base_ident: base_ident + 1,
            },
            base_ident,
        }
    }

    pub fn new_method_get_by_locale(
        method_name: Option<&'static str>,
        method_name_build_i18n_key: &str,
        instance_name_language_map: &str,
        arg_name_iso639: Option<&str>,
        arg_name_i18n_key: Option<&str>,
        base_ident: u8,
    ) -> Self {
        let method_name = method_name.unwrap_or("get_by_locale");
        let arg_name_iso639 = arg_name_iso639.unwrap_or("iso639_code");
        let arg_name_i18n_key = arg_name_i18n_key.unwrap_or("i18n_key");
        Self {
            name: method_name.to_string(),
            args: vec![
                Field {
                    r#type: Types::String,
                    name: arg_name_iso639.to_string(),
                    ..Default::default()
                },
                Field {
                    r#type: Types::String,
                    name: arg_name_i18n_key.to_string(),
                    ..Default::default()
                },
            ],
            return_type: Types::String,
            is_public: true,
            is_static: true,
            code: I18nMethodContent {
                tp: I18nMethodType::GetByLocale {
                    method_name_build_i18n_key: method_name_build_i18n_key.to_string(),
                    arg_name_iso639: arg_name_iso639.to_string(),
                    arg_name_i18n_key: arg_name_i18n_key.to_string(),
                    instance_name_language_map: instance_name_language_map.to_string(),
                },
                base_ident: base_ident + 1,
            },
            base_ident,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MethodCall {
    pub instance: Option<String>,
    pub method_name: String,
    pub args: Vec<Field>,
}
