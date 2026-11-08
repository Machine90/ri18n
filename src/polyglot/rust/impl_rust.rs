use std::{
    collections::{BTreeSet, HashSet},
    str::FromStr,
};

use crate::{
    common::{
        code_lines::CodeLines,
        constants::{
            ARG_NAME_ISO639, FIELD_NAME_CN_NAME, FIELD_NAME_EN_NAME, FIELD_NAME_NATIVE_NAME,
        },
        i18n_entry::I18nEntry,
    },
    polyglot::{
        common::{
            Naming,
            data_struct::DataStruct,
            field::Field,
            i18n_code_generator::{I18nCodeGenerator, InsertSupportLanguageListCall},
            i18n_method::{
                I18nMethodContent, I18nMethodType, Method, MethodCall, MethodDefinition,
            },
            i18n_switch::{I18nSwitch, LanguageMatcher},
            module::Module,
            package::{Package, PackageAction},
            rust_matcher,
            statement::{Operation, RightHand, Statement},
            to_constant, to_snake_case,
            used_types::Types,
        },
        rust::{IntoRust, IntoRustConstruct, RustSettings},
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
            Types::VarArgs => unreachable!(
                "VarArgs is not a Rust fn parameter; emit translate_with_args! macro instead"
            ),
            Types::Option(types) => format!("Option<{}>", types.into_rust(settings)),
        };
        code
    }
}

impl IntoRust for DataStruct {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let Self {
            name,
            fields,
            base_ident,
            ..
        } = self;
        let mut lines = CodeLines::new(*base_ident - 1, Some(","));
        lines.push_without_terminator("#[derive(Debug, Clone)]");
        lines.push_without_terminator(format!("pub struct {name} {{"));
        for field in fields {
            let field_name = field.name.replace(to_snake_case);
            let modifier = match field.is_public {
                true => format!("{} ", PUB_MODIFIER),
                _ => "".to_string(),
            };
            let type_name = field.r#type.into_rust(settings);
            lines.push(format!("\t{modifier}{}: {type_name}", field_name));
        }
        lines.push_without_terminator("}");
        lines.generate()
    }
}

impl IntoRustConstruct for DataStruct {
    fn construct_rust(&self, base_ident: u8, args: Vec<Naming>) -> String {
        let Self { name, fields, .. } = self;
        let fields_name = fields
            .iter()
            .map(|f| f.name.clone())
            .collect::<HashSet<_>>();

        let mut lines = CodeLines::new(base_ident, Some(","));
        lines.push_without_terminator(format!("{name} {{"));
        for arg in args {
            if !fields_name.contains(&arg) {
                panic!("Failed to construct method {name}, fields mismatch");
            }
            let arg_name = arg.replace(to_snake_case);
            lines.push(format!("\t{}: {}.to_string()", arg_name, arg_name));
        }
        lines.push_without_terminator("}");
        lines.generate()
    }
}

impl IntoRust for RightHand {
    fn into_rust(&self, _: &RustSettings) -> String {
        match self {
            RightHand::Value(v) => v.to_string(),
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
            I18nMethodType::Translate { language_matcher } => {
                lines.extend(language_matcher.into_rust(settings));
            }
            I18nMethodType::MatchEntries { match_entries } => {
                lines.extend(match_entries.into_rust(settings));
            }
            // Body is unused for with-args: Rust emits a macro, not a fn body.
            I18nMethodType::TranslateWithArgs { .. } | I18nMethodType::Empty => (),
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

/// Rust strategy for `TranslateWithArgs`: emit `translate_with_args!`
/// that delegates lookup to `translate` and formats via `formatx`.
fn macros_translate_with_args(module_path: String, method: &Method) -> String {
    let Method {
        definition, code, ..
    } = method;
    if !matches!(&code.tp, I18nMethodType::TranslateWithArgs { .. }) {
        return format!("");
    }
    let macros_name = &definition.name;
    let fn_name_translate = match &code.tp {
        I18nMethodType::TranslateWithArgs {
            fn_name_translate, ..
        } => fn_name_translate,
        _ => unreachable!(),
    };
    let macros_name = macros_name.replace(to_snake_case);
    let fn_name_translate = fn_name_translate.replace(to_snake_case);
    let module_path = format!("crate::{module_path}");

    format!(
        r#"
#[macro_export]
macro_rules! {macros_name} {{
    ($code:expr, $key:expr) => {{
        ${module_path}::{fn_name_translate}($code, $key)
    }};
    ($code:expr, $key:expr, $($values:tt)*) => {{{{
        #[allow(unused_imports)] use formatx::formatx;
        if let Some(template) = ${module_path}::{fn_name_translate}($code, $key) {{
            Some(formatx!(template, $($values)*))
        }} else {{
            None
        }}
    }}}};
}}"#,
    )
}

impl IntoRust for MethodDefinition {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let Self {
            name,
            args,
            return_type,
            is_public,
            is_static,
        } = self;
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
        format!(
            "{modifier}fn {}({args}) -> {}",
            name.replace(to_snake_case),
            return_type.into_rust(settings),
        )
    }
}

impl IntoRust for Method {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let Self {
            definition,
            code,
            base_ident,
        } = self;
        if matches!(&code.tp, I18nMethodType::TranslateWithArgs { .. }) {
            // Contract: with-args is a macro in Rust, not a function.
            return format!("");
        }

        let method_definition = definition.into_rust(settings);
        // there is no class in rust implement, so base_ident decrease 1
        let mut lines = CodeLines::new(*base_ident - 1, TERMINATOR);
        lines.push_without_terminator(format!("{method_definition} {{",));
        lines.extend(code.into_rust(settings));
        lines.push_without_terminator("}");
        lines.generate()
    }
}

impl IntoRust for I18nCodeGenerator {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let Self {
            struct_support_language_list,
            fn_translate_with_args,
            const_support_languages,
            call_insert_support_language_list,
            const_i18n_keys,
            fn_translate,
            fn_match_entries,
            package,
            name,
            ..
        } = self;

        let mut lines = CodeLines::new(0, TERMINATOR);

        let struct_support_language_codes = struct_support_language_list.into_rust(settings);
        lines
            .extend(struct_support_language_codes)
            .next_line_if_true(true);

        let codes = method_list_support_language(
            struct_support_language_list,
            call_insert_support_language_list,
            settings,
        );
        lines.extend(codes).next_line_if_true(true);

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

        if let Some(fn_translate_with_args) = fn_translate_with_args {
            if settings.formatx_enable {
                let module_path = package
                    .as_ref()
                    .map(|p| match &p {
                        PackageAction::Import(_) => None,
                        PackageAction::Define(package) => Some(package.clone()),
                    })
                    .flatten()
                    .map(|mut p| {
                        p.package_path = p.package_path.join(name);
                        p.into_rust(settings)
                    })
                    .unwrap_or_else(|| {
                        Package::from_str(&name)
                            .expect("Failed to parse path")
                            .into_rust(settings)
                    });
                // generate `translate_with_args!` macro (Rust with-args strategy)
                let codes = macros_translate_with_args(module_path, &fn_translate_with_args);
                lines.extend(codes).next_line_if_true(true);
            }
        }

        // generate method `translate()`
        if let Some(fn_translate) = fn_translate {
            let codes = fn_translate.into_rust(settings);
            lines.extend(codes).next_line_if_true(true);
        }

        // generate methods `match_entries_{iso639code}``
        for (_, method) in fn_match_entries {
            lines
                .extend(method.into_rust(settings))
                .next_line_if_true(true);
        }
        lines.generate()
    }
}

fn method_list_support_language(
    struct_support_language_list: &DataStruct,
    support_language: &BTreeSet<InsertSupportLanguageListCall>,
    settings: &RustSettings,
) -> String {
    let return_type = struct_support_language_list.name.as_str();
    let method_definition = MethodDefinition {
        name: Naming::new("list_support_language"),
        return_type: Types::List(Box::new(Types::Custom(return_type.to_string()))),
        is_public: true,
        is_static: true,
        ..Default::default()
    }
    .into_rust(settings);
    let mut lines = CodeLines::new(0, TERMINATOR);
    lines.push_without_terminator(format!("{method_definition} {{"));
    lines.push_without_terminator("\t[");
    for lang in support_language {
        let code = lang.language_iso639_code.clone();
        let cn_name = lang.iso639_entry.cn_name.to_string();
        let en_name = lang.iso639_entry.en_name.to_string();
        let native_name = lang.iso639_entry.native_name.to_string();
        lines.push_without_terminator(format!(
            "\t\t(\"{code}\", \"{cn_name}\", \"{en_name}\", \"{native_name}\"),"
        ));
    }
    let args = vec![
        Naming::new(ARG_NAME_ISO639),
        Naming::new(FIELD_NAME_CN_NAME),
        Naming::new(FIELD_NAME_EN_NAME),
        Naming::new(FIELD_NAME_NATIVE_NAME),
    ];
    let arg_list = args
        .iter()
        .map(|n| n.replace(to_snake_case))
        .collect::<Vec<_>>()
        .join(", ");
    let constructed = struct_support_language_list.construct_rust(3, args);
    lines
        .push_without_terminator("\t]")
        .push_without_terminator("\t.iter()")
        .push_without_terminator("\t.map(")
        .push_without_terminator(format!("\t\t|({arg_list})| {{"))
        .push_without_terminator(constructed)
        .push_without_terminator("\t\t}")
        .push_without_terminator("\t)")
        .push_without_terminator("\t.collect::<Vec<_>>()");
    lines.push_without_terminator("}");
    lines.generate()
}
