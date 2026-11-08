use crate::common::code_lines::CodeLines;
use crate::polyglot::common::i18n_method::MethodDefinition;
use crate::polyglot::common::i18n_switch::{I18nSwitch, LanguageMatcher};
use crate::polyglot::common::{Naming, to_camelcase, to_constant, typescript_matcher};
use crate::polyglot::{
    common::{
        data_struct::DataStruct,
        field::Field,
        i18n_code_generator::{I18nCodeGenerator, InsertSupportLanguageListCall},
        i18n_method::{I18nMethodContent, I18nMethodType, Method, MethodCall},
        package::PackageAction,
        statement::{Operation, RightHand, Statement},
        used_types::{Construct, Types},
    },
    typescript::IntoTypeScript,
};

const TERMINATOR: Option<&'static str> = Some(";");
const CONSTANTS_NAME_KEY: &str = "Keys";

/// IR `base_ident` assumes an enclosing type (Java). TS emits a module at file scope.
fn ts_file_ident(base_ident: u8) -> u8 {
    base_ident.saturating_sub(1)
}

impl IntoTypeScript for Types {
    fn into_typescript(&self) -> String {
        match self {
            Types::String | Types::RefString => "string".to_string(),
            Types::List(inner) => format!("Array<{}>", inner.into_typescript()),
            Types::HashMap(k, v) => {
                format!("Record<{}, {}>", k.into_typescript(), v.into_typescript())
            }
            Types::Option(inner) => format!("{} | null", inner.into_typescript()),
            Types::Custom(tp) => tp.to_string(),
            Types::Void => "void".to_string(),
            Types::VarArgs => "...args: unknown[]".to_string(),
        }
    }
}

impl IntoTypeScript for Field {
    fn into_typescript(&self) -> String {
        let Self {
            is_final,
            is_static,
            r#type,
            name,
            ..
        } = self;
        let name = if *is_final && *is_static {
            name.replace(to_constant)
        } else {
            name.replace(to_camelcase)
        };
        // VarArgs already includes `...args: type`
        if matches!(r#type, Types::VarArgs) {
            return r#type.into_typescript().replacen("args", &name, 1);
        }
        format!("{name}: {}", r#type.into_typescript())
    }
}

impl IntoTypeScript for RightHand {
    fn into_typescript(&self) -> String {
        match self {
            RightHand::Value(v) => v.to_string(),
            RightHand::Construct(c) => c.into_typescript(),
        }
    }
}

impl IntoTypeScript for Construct {
    fn into_typescript(&self) -> String {
        let Self { tp, args } = self;
        match tp {
            Types::List(_) => "[]".to_string(),
            _ => {
                let args = args
                    .iter()
                    .map(|f| f.into_typescript())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{ {} }}", args)
            }
        }
    }
}

impl IntoTypeScript for Operation {
    fn into_typescript(&self) -> String {
        self.to_string()
    }
}

impl IntoTypeScript for Statement {
    fn into_typescript(&self) -> String {
        let Self {
            left_hand,
            op,
            right_hand,
        } = self;
        // Constants: `export const NAME = "value"`
        if left_hand.is_static && left_hand.is_final {
            return format!(
                "export const {} {} {}",
                left_hand.name.replace(to_constant),
                op.into_typescript(),
                right_hand.into_typescript()
            );
        }
        format!(
            "{} {} {}",
            left_hand.into_typescript(),
            op.into_typescript(),
            right_hand.into_typescript()
        )
    }
}

impl IntoTypeScript for DataStruct {
    fn into_typescript(&self) -> String {
        let Self {
            name,
            fields,
            base_ident,
            ..
        } = self;
        let mut lines = CodeLines::new(ts_file_ident(*base_ident), TERMINATOR);
        lines.push_without_terminator(format!("export interface {name} {{"));
        for field in fields {
            lines.push_without_terminator(format!(
                "\t{}: {};",
                field.name.replace(to_camelcase),
                match &field.r#type {
                    Types::String | Types::RefString => "string".to_string(),
                    other => other.into_typescript(),
                }
            ));
        }
        lines.push_without_terminator("}");
        lines.generate()
    }
}

impl IntoTypeScript for I18nSwitch {
    fn into_typescript(&self) -> String {
        let Self {
            arg_name_i18n_key,
            entries,
            base_ident,
        } = self;
        let mut lines = CodeLines::new(ts_file_ident(*base_ident), TERMINATOR);
        lines.push_without_terminator(format!(
            "switch ({}) {{",
            arg_name_i18n_key.replace(to_camelcase)
        ));
        for (key, value) in entries {
            let const_name = Naming::new(key.replace(".", "_")).replace(to_constant);
            lines.push_without_terminator(format!("\tcase {CONSTANTS_NAME_KEY}.{const_name}:"));
            lines.push(format!(
                "\t\treturn \"{}\"",
                value.replace(typescript_matcher)
            ));
        }
        lines.push_without_terminator("\tdefault:");
        lines.push(format!("\t\treturn null"));
        lines.push_without_terminator("}");
        lines.generate()
    }
}

impl IntoTypeScript for LanguageMatcher {
    fn into_typescript(&self) -> String {
        let Self {
            language_to_fn_match_entries,
            base_ident,
            arg_name_iso639,
        } = self;
        let mut lines = CodeLines::new(ts_file_ident(*base_ident), TERMINATOR);
        for (lang, call) in language_to_fn_match_entries {
            lines.push_without_terminator(format!(
                "if ({} === \"{lang}\") {{",
                arg_name_iso639.replace(to_camelcase)
            ));
            lines.push(format!("\treturn {}", call.into_typescript()));
            lines.push_without_terminator("}");
        }
        lines.generate()
    }
}

impl IntoTypeScript for MethodCall {
    fn into_typescript(&self) -> String {
        let Self {
            instance,
            raw_instance_name,
            method_name,
            args_name,
            raw_args_name,
        } = self;
        let instance_name = instance
            .clone()
            .map(|name| name.replace(to_camelcase))
            .map(|name| format!("{name}."));
        let raw_instance_name = raw_instance_name.clone().map(|name| format!("{name}."));
        let args_str = args_name
            .iter()
            .map(|arg_name| arg_name.replace(to_camelcase))
            .collect::<Vec<_>>()
            .join(", ");
        let raw_args_name = raw_args_name.join(", ");
        let args_name = if raw_args_name.is_empty() {
            args_str
        } else {
            raw_args_name
        };
        let prefix = raw_instance_name.or(instance_name).unwrap_or_default();
        format!("{prefix}{}({args_name})", method_name.replace(to_camelcase))
    }
}

impl IntoTypeScript for I18nMethodContent {
    fn into_typescript(&self) -> String {
        let Self { tp, base_ident } = self;
        let mut lines = CodeLines::new(ts_file_ident(*base_ident), None);
        match tp {
            I18nMethodType::Translate { language_matcher } => {
                lines
                    .extend(language_matcher.into_typescript())
                    .push(format!("return null"));
            }
            I18nMethodType::TranslateWithArgs {
                arg_name_iso639,
                arg_name_i18n_key,
                arg_name_args,
                fn_name_translate,
            } => {
                let template = format!("{}Template", arg_name_i18n_key.replace(to_camelcase));
                lines
                    .push(format!(
                        "const {template} = {}({}, {})",
                        fn_name_translate.replace(to_camelcase),
                        arg_name_iso639.replace(to_camelcase),
                        arg_name_i18n_key.replace(to_camelcase)
                    ))
                    .push_without_terminator(format!(
                        "if ({template} == null || {template} === \"\") {{"
                    ))
                    .push(format!("\treturn null"))
                    .push_without_terminator("}")
                    .push(format!(
                        "return formatTemplate({template}, ...{})",
                        arg_name_args.replace(to_camelcase)
                    ));
            }
            I18nMethodType::MatchEntries { match_entries } => {
                lines.extend(match_entries.into_typescript());
            }
            I18nMethodType::Empty => (),
        }
        lines.generate()
    }
}

impl IntoTypeScript for MethodDefinition {
    fn into_typescript(&self) -> String {
        let Self {
            name,
            args,
            return_type,
            is_public,
            ..
        } = self;
        let export = if *is_public { "export " } else { "" };
        let args = args
            .iter()
            .map(IntoTypeScript::into_typescript)
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "{export}function {}({args}): {}",
            name.replace(to_camelcase),
            return_type.into_typescript()
        )
    }
}

impl IntoTypeScript for Method {
    fn into_typescript(&self) -> String {
        let Self {
            definition,
            code,
            base_ident,
        } = self;
        let mut lines = CodeLines::new(ts_file_ident(*base_ident), TERMINATOR);
        lines.push_without_terminator(format!("{} {{", definition.into_typescript()));
        lines.extend(code.into_typescript());
        lines.push_without_terminator("}");
        lines.generate()
    }
}

fn format_helper() -> String {
    r#"function formatTemplate(template: string, ...args: unknown[]): string {
	let i = 0;
	return template.replace(/\{\}/g, () => String(args[i++]));
}"#
    .to_string()
}

impl IntoTypeScript for I18nCodeGenerator {
    fn into_typescript(&self) -> String {
        let Self {
            package,
            struct_support_language_list,
            fn_translate,
            fn_translate_with_args,
            const_support_languages,
            call_insert_support_language_list,
            const_i18n_keys,
            fn_match_entries,
            ..
        } = self;

        let mut lines = CodeLines::new(0, TERMINATOR);

        if let Some(PackageAction::Define(pkg) | PackageAction::Import(pkg)) = package {
            let path = pkg
                .package_path
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join("/");
            lines.push_without_terminator(format!("// module path: {path}"));
            lines.next_line_if_true(true);
        }

        lines
            .extend(struct_support_language_list.into_typescript())
            .next_line_if_true(true);

        // listSupportLanguage
        lines.push_without_terminator(
            "export function listSupportLanguage(): Array<SupportLanguage> {",
        );
        lines.push_without_terminator("\treturn [");
        for InsertSupportLanguageListCall {
            language_iso639_code,
            iso639_entry,
        } in call_insert_support_language_list.iter()
        {
            lines.push_without_terminator(format!(
                "\t\t{{ iso639Code: \"{language_iso639_code}\", cnName: \"{}\", enName: \"{}\", nativeName: \"{}\" }},",
                iso639_entry.cn_name, iso639_entry.en_name, iso639_entry.native_name
            ));
        }
        lines.push_without_terminator("\t];");
        lines.push_without_terminator("}").next_line_if_true(true);

        // Iso639Codes object
        lines.push_without_terminator("export const Iso639Codes = {");
        for statement in const_support_languages {
            let name = statement.left_hand.name.replace(to_constant);
            let value = match &statement.right_hand {
                RightHand::Value(v) => v.clone(),
                RightHand::Construct(_) => "\"\"".to_string(),
            };
            lines.push_without_terminator(format!("\t{name}: {value},"));
        }
        lines.push_without_terminator("} as const;");
        lines.next_line_if_true(true);

        // Keys object
        lines.push_without_terminator(format!("export const {CONSTANTS_NAME_KEY} = {{"));
        for statement in const_i18n_keys {
            let name = statement.left_hand.name.replace(to_constant);
            let value = match &statement.right_hand {
                RightHand::Value(v) => v.clone(),
                RightHand::Construct(_) => "\"\"".to_string(),
            };
            lines.push_without_terminator(format!("\t{name}: {value},"));
        }
        lines.push_without_terminator("} as const;");
        lines.next_line_if_true(true);

        if fn_translate_with_args.is_some() {
            lines.extend(format_helper()).next_line_if_true(true);
        }

        if let Some(fn_translate_with_args) = fn_translate_with_args {
            lines
                .extend(fn_translate_with_args.into_typescript())
                .next_line_if_true(true);
        }

        if let Some(fn_translate) = fn_translate {
            lines
                .extend(fn_translate.into_typescript())
                .next_line_if_true(true);
        }

        for (_, method) in fn_match_entries {
            lines
                .extend(method.into_typescript())
                .next_line_if_true(true);
        }

        lines.generate()
    }
}
