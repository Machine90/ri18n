use std::str::FromStr;

use crate::{
    common::{code_lines::CodeLines, i18n_entry::I18nEntry},
    polyglot::{
        common::{
            Naming,
            field::Field,
            i18n_code_generator::I18nCodeGenerator,
            i18n_method::{I18nMethodContent, I18nMethodType, Method, MethodCall},
            i18n_switch::{I18nSwitch, LanguageMatcher},
            module::Module,
            package::{Package, PackageAction},
            rust_matcher,
            statement::{Operation, RightHand, Statement},
            to_constant, to_snake_case,
            used_types::Types,
        },
        rust::{IntoRust, RustSettings},
    },
};

const TERMINATOR: Option<&'static str> = Some(";");
const CONSTANTS_NAME_KEY: &str = "keys";
const CONSTANTS_NAME_ISO639_CODES: &str = "iso639_codes";
const PUB_MODIFIER: &str = "pub";

impl IntoRust for Package {
    fn into_rust(&self, _: &RustSettings) -> String {
        let package = self
            .package_path
            .into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("::");
        package
    }
}

impl IntoRust for PackageAction {
    fn into_rust(&self, settings: &RustSettings) -> String {
        match self {
            PackageAction::Import(package) => format!("use {}", package.into_rust(settings)),
            _ => panic!("Rust doesn't support to define package in file"),
        }
    }
}

impl IntoRust for Field {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let Self {
            is_public,
            is_static,
            is_final,
            r#type,
            name,
        } = self;
        let modifier = match (is_public, is_static, is_final) {
            (true, true, true) => format!("{PUB_MODIFIER} const "),
            (true, true, false) => format!("{PUB_MODIFIER} static "),
            (true, false, true) => format!("{PUB_MODIFIER} "),
            (true, false, false) => format!("{PUB_MODIFIER} "),
            (false, true, true) => format!("const "),
            (false, true, false) => format!("static "),
            (false, false, true) => format!(""),
            (false, false, false) => format!(""),
        };
        let name = if *is_final && *is_static {
            name.replace(to_constant)
        } else {
            name.replace(to_snake_case)
        };
        format!("{modifier}{name}: {}", r#type.into_rust(settings))
    }
}

impl IntoRust for Types {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let code = match self {
            Types::String => "String".to_string(),
            Types::RefString => "&str".to_string(),
            Types::List(generic_type) => {
                format!("Vec<{}>", generic_type.into_rust(settings))
            }
            Types::HashMap(generic_key, generic_value) => {
                format!(
                    "std::collections::HashMap<{}, {}>",
                    generic_key.into_rust(settings),
                    generic_value.into_rust(settings)
                )
            }
            Types::Custom(tp) => tp.to_string(),
            Types::Void => "()".to_string(),
            Types::ListAny => unreachable!("rust doesn't support ..args as method input"),
            Types::Option(types) => format!("Option<{}>", types.into_rust(settings)),
        };
        code
    }
}

impl IntoRust for RightHand {
    fn into_rust(&self, _: &RustSettings) -> String {
        match self {
            RightHand::Value(v) => v.to_string(),
            // RightHand::Construct(construct) => construct.into_java(),
            _ => "".to_string(),
        }
    }
}

impl IntoRust for Operation {
    fn into_rust(&self, _: &RustSettings) -> String {
        self.to_string()
    }
}

impl IntoRust for Statement {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let Self {
            left_hand,
            op,
            right_hand,
        } = self;
        format!(
            "{} {} {}",
            left_hand.into_rust(settings),
            op.into_rust(settings),
            right_hand.into_rust(settings),
        )
    }
}

impl IntoRust for Module {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let Self {
            is_public,
            name,
            statements,
            base_ident,
        } = self;

        let modifier = match is_public {
            true => format!("{PUB_MODIFIER} "),
            _ => "".to_string(),
        };
        let mut lines = CodeLines::new(*base_ident - 1, TERMINATOR);
        lines.push_without_terminator(format!("{modifier}mod {name} {{"));
        for statement in statements {
            lines.push(format!("\t{}", statement.into_rust(settings)));
        }
        lines.push_without_terminator("}");
        lines.generate()
    }
}

impl IntoRust for I18nSwitch {
    fn into_rust(&self, _: &RustSettings) -> String {
        let Self {
            arg_name_i18n_key,
            entries,
            base_ident,
        } = self;
        let mut lines = CodeLines::new(*base_ident - 1, TERMINATOR);
        const RETURN_VALUE_NAME: &str = "content";
        lines.push_without_terminator(format!(
            "let {RETURN_VALUE_NAME} = match {} {{",
            arg_name_i18n_key.replace(to_snake_case)
        ));
        for I18nEntry { key, value } in entries {
            let key = key.replace(".", "_");
            lines.push_without_terminator(format!(
                "\t{CONSTANTS_NAME_KEY}::{} => \"{}\",",
                Naming::new(key).replace(to_constant),
                value.replace(rust_matcher)
            ));
        }
        lines.push_without_terminator(format!("\t_ => return None,"));
        lines
            .push_without_terminator("}")
            .push(".to_string()")
            .push_without_terminator(format!("Some({RETURN_VALUE_NAME})"))
            .generate()
    }
}

impl IntoRust for LanguageMatcher {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let Self {
            arg_name_iso639,
            language_to_fn_match_entries,
            base_ident,
        } = self;
        let mut lines = CodeLines::new(*base_ident - 1, TERMINATOR);
        lines.push_without_terminator(format!(
            "match {} {{",
            arg_name_iso639.replace(to_snake_case)
        ));
        for (language, fn_match_entries_call) in language_to_fn_match_entries {
            lines.push_without_terminator(format!(
                "\t\"{language}\" => {},",
                fn_match_entries_call.into_rust(settings)
            ));
        }
        lines
            .push_without_terminator(format!("\t_ => None,"))
            .push_without_terminator("}");
        lines.generate()
    }
}

impl IntoRust for I18nMethodContent {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let Self { tp, base_ident } = self;
        let mut lines = CodeLines::new(*base_ident - 1, None);
        match tp {
            I18nMethodType::GetByLocale { language_matcher } => {
                lines.extend(language_matcher.into_rust(settings));
            }
            I18nMethodType::MatchEntries { match_entries } => {
                lines.extend(match_entries.into_rust(settings));
            }
            I18nMethodType::GetByLocaleWithArgs { .. } | I18nMethodType::Empty => (),
        };
        lines.generate()
    }
}

impl IntoRust for MethodCall {
    fn into_rust(&self, _: &RustSettings) -> String {
        let Self {
            instance,
            raw_instance_name,
            method_name,
            args_name,
            raw_args_name,
        } = self;
        let instance_name = instance
            .clone()
            .map(|name| name.replace(to_snake_case))
            .map(|name| format!("{name}."));
        let raw_instance_name = raw_instance_name.clone().map(|name| format!("{name}."));
        let args_str = args_name
            .iter()
            .map(|arg_name| arg_name.replace(to_snake_case))
            .collect::<Vec<_>>()
            .join(", ");
        let raw_args_name = raw_args_name.join(", ");
        let args_name = if raw_args_name.is_empty() {
            args_str
        } else {
            raw_args_name
        };

        let instance_call_with = raw_instance_name.or(instance_name).unwrap_or_default();
        let method_name = method_name.replace(to_snake_case);

        format!("{instance_call_with}{method_name}({args_name})")
    }
}

fn macros_get_by_locale(macros_name: &Naming, fn_name_get_locale: &Naming) -> String {
    let macros_name = macros_name.replace(to_snake_case);
    let fn_name_get_locale = fn_name_get_locale.replace(to_snake_case);
    format!(
        r#"
#[macro_export]
macro_rules! {macros_name} {{
    ($code:expr, $key:expr) => {{
        $crate::{fn_name_get_locale}($code, $key)
    }};
    ($code:expr, $key:expr, $($values:tt)*) => {{
        if let Some(template) = $crate::{fn_name_get_locale}($code, $key) {{
            Some($crate::formatx!(template, $($values)*))
        }} else {{
            None
        }}
    }};
}}"#,
    )
}

impl IntoRust for Method {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let Self {
            name,
            args,
            return_type,
            is_public,
            is_static,
            code,
            base_ident,
        } = self;
        if matches!(&code.tp, I18nMethodType::GetByLocaleWithArgs { .. }) {
            // generate macros
            let fn_name_get_locale = match &code.tp {
                I18nMethodType::GetByLocaleWithArgs {
                    fn_name_get_locale, ..
                } => fn_name_get_locale,
                _ => unreachable!(),
            };
            return macros_get_by_locale(name, fn_name_get_locale);
        }
        let modifier = match is_public {
            true => format!("{PUB_MODIFIER} "),
            _ => format!(""),
        };
        let mut args = args.clone();
        match is_static {
            false => args.push(Field {
                name: Naming::new("&self"),
                ..Default::default()
            }),
            _ => (),
        };
        let args = args
            .iter()
            .map(|arg| arg.into_rust(settings))
            .collect::<Vec<_>>()
            .join(", ");
        // there is no class in rust implement, so base_ident decrease 1
        let mut lines = CodeLines::new(*base_ident - 1, TERMINATOR);
        lines.push_without_terminator(format!(
            "{modifier}fn {}({args}) -> {} {{",
            name.replace(to_snake_case),
            return_type.into_rust(settings),
        ));
        lines.extend(code.into_rust(settings));
        lines.push_without_terminator("}");
        lines.generate()
    }
}

impl IntoRust for I18nCodeGenerator {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let Self {
            struct_support_language_list,
            fn_get_by_locale_with_args,
            const_support_languages,
            stat_build_support_language_list,
            call_insert_support_language_list,
            const_i18n_keys,
            fn_get_by_locale,
            fn_match_entries,
            ..
        } = self;
        let mut use_crates = vec![];
        if settings.formatx_enable {
            use_crates.push(PackageAction::Import(
                Package::from_str("formatx::formatx").expect("Failed to parse package"),
            ));
        }

        let mut lines = CodeLines::new(0, TERMINATOR);
        lines
            .push_without_terminator("#![allow(unused)]")
            .next_line_if_true(true);
        let has_crates = !use_crates.is_empty();
        for use_crate in use_crates {
            lines.push(use_crate.into_rust(settings));
        }
        lines.next_line_if_true(has_crates);

        // render iso639 codes constants
        let mut iso639_module = Module {
            is_public: true,
            name: CONSTANTS_NAME_ISO639_CODES.to_string(),
            base_ident: 1,
            ..Default::default()
        };
        for statement in const_support_languages {
            iso639_module.statements.insert(statement.clone());
        }
        lines
            .extend(iso639_module.into_rust(settings))
            .next_line_if_true(true);

        // render keys constants
        let mut key_module = Module {
            is_public: true,
            base_ident: 1,
            name: CONSTANTS_NAME_KEY.to_string(),
            ..Default::default()
        };
        for const_key in const_i18n_keys {
            key_module.statements.insert(const_key.clone());
        }
        lines
            .extend(key_module.into_rust(settings))
            .next_line_if_true(true);

        if settings.formatx_enable {
            // generate `get_by_locale!()` macros
            let codes = fn_get_by_locale_with_args.into_rust(settings);
            lines.extend(codes).next_line_if_true(true);
        }

        // generate method `get_by_locale()`
        let codes = fn_get_by_locale.into_rust(settings);
        lines.extend(codes).next_line_if_true(true);

        // generate methods `match_entries_{iso639code}``
        for (_, method) in fn_match_entries {
            lines
                .extend(method.into_rust(settings))
                .next_line_if_true(true);
        }
        lines.generate()
    }
}
