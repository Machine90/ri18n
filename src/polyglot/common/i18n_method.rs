use crate::{
    common::constants::{
        ARG_NAME_ARGS, ARG_NAME_I18N_KEY, ARG_NAME_ISO639, METHOD_NAME_GET_BY_LOCALE,
        METHOD_NAME_GET_BY_LOCALE_WITH_ARGS,
    },
    polyglot::common::{Naming, field::Field, i18n_switch::LanguageMatcher, used_types::Types},
};

#[derive(Debug, Default)]
pub enum I18nMethodType {
    GetByLocale {
        language_matcher: LanguageMatcher,
    },
    GetByLocaleWithArgs {
        fn_name_get_locale: Naming,
        arg_name_iso639: Naming,
        arg_name_i18n_key: Naming,
        arg_name_args: Naming,
    },
    #[default]
    Empty,
}

#[derive(Debug, Default)]
pub struct I18nMethodContent {
    pub tp: I18nMethodType,
    pub base_ident: u8,
}

#[derive(Debug, Default)]
pub struct Method {
    pub name: Naming,
    pub args: Vec<Field>,
    pub return_type: Types,
    pub is_public: bool,
    pub is_static: bool,
    pub code: I18nMethodContent,
    pub base_ident: u8,
}

impl Method {
    pub fn new_method_get_by_locale(
        arg_name_iso639: Option<&str>,
        arg_name_i18n_key: Option<&str>,
        base_ident: u8,
    ) -> Self {
        let method_name = METHOD_NAME_GET_BY_LOCALE;
        let arg_name_iso639 = arg_name_iso639.unwrap_or(ARG_NAME_ISO639);
        let arg_name_i18n_key = arg_name_i18n_key.unwrap_or(ARG_NAME_I18N_KEY);
        Self {
            name: Naming::new(method_name),
            args: vec![
                Field {
                    r#type: Types::String,
                    name: Naming::new(arg_name_iso639),
                    ..Default::default()
                },
                Field {
                    r#type: Types::String,
                    name: Naming::new(arg_name_i18n_key),
                    ..Default::default()
                },
            ],
            return_type: Types::String,
            is_public: true,
            is_static: true,
            code: I18nMethodContent {
                tp: I18nMethodType::GetByLocale {
                    language_matcher: LanguageMatcher {
                        arg_name_iso639: Naming::new(arg_name_iso639),
                        arg_name_i18n_key: Naming::new(arg_name_i18n_key),
                        language_to_entries: Default::default(),
                        base_ident: base_ident + 1,
                    },
                },
                base_ident: base_ident + 1,
            },
            base_ident,
        }
    }

    pub fn new_method_get_by_locale_with_args(
        arg_name_iso639: Option<&str>,
        arg_name_i18n_key: Option<&str>,
        arg_name_args: Option<&str>,
        base_ident: u8,
    ) -> Self {
        let method_name = METHOD_NAME_GET_BY_LOCALE_WITH_ARGS;
        let arg_name_iso639 = arg_name_iso639.unwrap_or(ARG_NAME_ISO639);
        let arg_name_i18n_key = arg_name_i18n_key.unwrap_or(ARG_NAME_I18N_KEY);
        let arg_name_args = arg_name_args.unwrap_or(ARG_NAME_ARGS);
        Self {
            name: Naming::new(method_name),
            args: vec![
                Field {
                    r#type: Types::String,
                    name: Naming::new(arg_name_iso639),
                    ..Default::default()
                },
                Field {
                    r#type: Types::String,
                    name: Naming::new(arg_name_i18n_key),
                    ..Default::default()
                },
                Field {
                    r#type: Types::ListAny,
                    name: Naming::new(arg_name_args),
                    ..Default::default()
                },
            ],
            return_type: Types::String,
            is_public: true,
            is_static: true,
            code: I18nMethodContent {
                tp: I18nMethodType::GetByLocaleWithArgs {
                    fn_name_get_locale: Naming::new(METHOD_NAME_GET_BY_LOCALE),
                    arg_name_iso639: Naming::new(arg_name_iso639),
                    arg_name_i18n_key: Naming::new(arg_name_i18n_key),
                    arg_name_args: Naming::new(arg_name_args),
                },
                base_ident: base_ident + 1,
            },
            base_ident,
        }
    }

    pub fn insert(&mut self, language_iso639_code: &str, key: &str, value: &str) {
        match &mut self.code.tp {
            I18nMethodType::GetByLocale { language_matcher } => {
                language_matcher.insert(language_iso639_code, key, value);
            }
            _ => (),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MethodCall {
    pub instance: Option<String>,
    pub method_name: String,
    pub args: Vec<String>,
}
