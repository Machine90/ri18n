use std::hash::Hash;

use crate::{
    common::constants::{
        ARG_NAME_ARGS, ARG_NAME_I18N_KEY, ARG_NAME_ISO639, METHOD_NAME_TRANSLATE,
        METHOD_NAME_TRANSLATE_WITH_ARGS,
    },
    polyglot::common::{
        Naming,
        field::Field,
        i18n_switch::{I18nSwitch, LanguageMatcher},
        used_types::Types,
    },
};

#[derive(Debug, Default)]
pub enum I18nMethodType {
    /// Plain locale+key lookup. Method name: `translate`.
    Translate {
        language_matcher: LanguageMatcher,
    },
    /// Lookup then format with variadic args.
    ///
    /// IR method name is always `translate_with_args` (distinct from plain lookup).
    /// Backend strategies:
    /// - Java: emit a real static method with `Object... args`
    /// - TypeScript / Python: emit a function that formats `{}` placeholders
    /// - Rust: emit `translate_with_args!` macro when formatx is enabled; otherwise omit
    TranslateWithArgs {
        fn_name_translate: Naming,
        arg_name_iso639: Naming,
        arg_name_i18n_key: Naming,
        arg_name_args: Naming,
    },
    MatchEntries {
        match_entries: I18nSwitch,
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
pub struct MethodDefinition {
    pub name: Naming,
    pub args: Vec<Field>,
    pub return_type: Types,
    pub is_public: bool,
    pub is_static: bool,
}

#[derive(Debug, Default)]
pub struct Method {
    pub definition: MethodDefinition,
    pub code: I18nMethodContent,
    pub base_ident: u8,
}

impl Method {
    pub fn new_method_translate(
        arg_name_iso639: Option<&str>,
        arg_name_i18n_key: Option<&str>,
        base_ident: u8,
    ) -> Self {
        let method_name = METHOD_NAME_TRANSLATE;
        let arg_name_iso639 = arg_name_iso639.unwrap_or(ARG_NAME_ISO639);
        let arg_name_i18n_key = arg_name_i18n_key.unwrap_or(ARG_NAME_I18N_KEY);
        Self {
            definition: MethodDefinition {
                name: Naming::new(method_name),
                args: vec![
                    Field {
                        r#type: Types::RefString,
                        name: Naming::new(arg_name_iso639),
                        ..Default::default()
                    },
                    Field {
                        r#type: Types::RefString,
                        name: Naming::new(arg_name_i18n_key),
                        ..Default::default()
                    },
                ],
                return_type: Types::Option(Box::new(Types::String)),
                is_public: true,
                is_static: true,
            },
            code: I18nMethodContent {
                tp: I18nMethodType::Translate {
                    language_matcher: LanguageMatcher {
                        arg_name_iso639: Naming::new(arg_name_iso639),
                        language_to_fn_match_entries: Default::default(),
                        base_ident: base_ident + 1,
                    },
                },
                base_ident: base_ident + 1,
            },
            base_ident,
        }
    }

    pub fn new_method_translate_with_args(
        arg_name_iso639: Option<&str>,
        arg_name_i18n_key: Option<&str>,
        arg_name_args: Option<&str>,
        base_ident: u8,
    ) -> Self {
        let method_name = METHOD_NAME_TRANSLATE_WITH_ARGS;
        let arg_name_iso639 = arg_name_iso639.unwrap_or(ARG_NAME_ISO639);
        let arg_name_i18n_key = arg_name_i18n_key.unwrap_or(ARG_NAME_I18N_KEY);
        let arg_name_args = arg_name_args.unwrap_or(ARG_NAME_ARGS);
        Self {
            definition: MethodDefinition {
                name: Naming::new(method_name),
                args: vec![
                    Field {
                        r#type: Types::RefString,
                        name: Naming::new(arg_name_iso639),
                        ..Default::default()
                    },
                    Field {
                        r#type: Types::RefString,
                        name: Naming::new(arg_name_i18n_key),
                        ..Default::default()
                    },
                    Field {
                        r#type: Types::VarArgs,
                        name: Naming::new(arg_name_args),
                        ..Default::default()
                    },
                ],
                return_type: Types::Option(Box::new(Types::String)),
                is_public: true,
                is_static: true,
            },
            code: I18nMethodContent {
                tp: I18nMethodType::TranslateWithArgs {
                    fn_name_translate: Naming::new(METHOD_NAME_TRANSLATE),
                    arg_name_iso639: Naming::new(arg_name_iso639),
                    arg_name_i18n_key: Naming::new(arg_name_i18n_key),
                    arg_name_args: Naming::new(arg_name_args),
                },
                base_ident: base_ident + 1,
            },
            base_ident,
        }
    }

    pub fn new_method_match_entries(
        iso639_code: &str,
        arg_name_i18n_key: Option<&str>,
        base_ident: u8,
    ) -> Self {
        let method_name = Naming::new(&format!("match_entries_{iso639_code}"));
        let arg_name_i18n_key = arg_name_i18n_key.unwrap_or(ARG_NAME_I18N_KEY);
        Self {
            definition: MethodDefinition {
                name: method_name,
                args: vec![Field {
                    r#type: Types::RefString,
                    name: Naming::new(arg_name_i18n_key),
                    ..Default::default()
                }],
                return_type: Types::Option(Box::new(Types::String)),
                is_public: false,
                is_static: true,
            },
            code: I18nMethodContent {
                tp: I18nMethodType::MatchEntries {
                    match_entries: I18nSwitch {
                        arg_name_i18n_key: Naming::new(arg_name_i18n_key),
                        base_ident: base_ident + 1,
                        ..Default::default()
                    },
                },
                base_ident: base_ident + 1,
            },
            base_ident,
        }
    }

    pub fn insert_match_entries(&mut self, key: &str, value: &str) {
        match &mut self.code.tp {
            I18nMethodType::MatchEntries { match_entries } => {
                match_entries.insert(key, value);
            }
            _ => (),
        }
    }

    pub fn insert_translate(&mut self, language_iso639_code: &str, fn_match_entries: &Method) {
        match &mut self.code.tp {
            I18nMethodType::Translate { language_matcher } => {
                language_matcher.insert(language_iso639_code, fn_match_entries);
            }
            _ => (),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MethodCall {
    pub instance: Option<Naming>,
    pub raw_instance_name: Option<String>,
    pub method_name: Naming,
    pub args_name: Vec<Naming>,
    pub raw_args_name: Vec<String>,
}

impl From<&Method> for MethodCall {
    fn from(method: &Method) -> Self {
        let Method {
            definition:
                MethodDefinition {
                    name,
                    args,
                    return_type,
                    is_public,
                    is_static,
                },
            code,
            base_ident,
        } = method;
        let args_name = args.iter().map(|a| a.name.clone()).collect::<Vec<_>>();
        MethodCall {
            instance: None,
            method_name: name.clone(),
            args_name,
            ..Default::default()
        }
    }
}

impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        self.definition.name == other.definition.name
    }
}

impl Eq for Method {}

impl Hash for Method {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.definition.name.hash(state);
    }
}
