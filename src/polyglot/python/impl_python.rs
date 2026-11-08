use crate::common::code_lines::CodeLines;
use crate::polyglot::common::i18n_method::MethodDefinition;
use crate::polyglot::common::i18n_switch::{I18nSwitch, LanguageMatcher};
use crate::polyglot::common::{Naming, python_matcher, to_constant, to_snake_case};
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
    python::IntoPython,
};

/// Empty terminator: Python statements do not use `;`.
const TERMINATOR: Option<&'static str> = Some("");
const CONSTANTS_NAME_KEY: &str = "Keys";

/// IR `base_ident` assumes an enclosing type (Java). Python emits a module at file scope.
fn py_file_ident(base_ident: u8) -> u8 {
    base_ident.saturating_sub(1)
}

/// Keep IR names that are already snake_case (e.g. `iso639_code`); otherwise convert.
fn python_ident(name: &Naming) -> String {
    let raw = &name.name;
    if raw
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        raw.clone()
    } else {
        name.replace(to_snake_case)
    }
}

fn py_string_literal(s: &str) -> String {
    format!(
        "\"{}\"",
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    )
}

impl IntoPython for Types {
    fn into_python(&self) -> String {
        match self {
            Types::String | Types::RefString => "str".to_string(),
            Types::List(inner) => format!("list[{}]", inner.into_python()),
            Types::HashMap(k, v) => {
                format!("dict[{}, {}]", k.into_python(), v.into_python())
            }
            Types::Option(inner) => format!("{} | None", inner.into_python()),
            Types::Custom(tp) => tp.to_string(),
            Types::Void => "None".to_string(),
            Types::VarArgs => "*args: object".to_string(),
        }
    }
}

impl IntoPython for Field {
    fn into_python(&self) -> String {
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
            python_ident(name)
        };
        if matches!(r#type, Types::VarArgs) {
            return r#type.into_python().replacen("args", &name, 1);
        }
        format!("{name}: {}", r#type.into_python())
    }
}

impl IntoPython for RightHand {
    fn into_python(&self) -> String {
        match self {
            RightHand::Value(v) => v.to_string(),
            RightHand::Construct(c) => c.into_python(),
        }
    }
}

impl IntoPython for Construct {
    fn into_python(&self) -> String {
        let Self { tp, args } = self;
        match tp {
            Types::List(_) => "[]".to_string(),
            Types::Custom(name) => {
                let args = args
                    .iter()
                    .map(|f| f.into_python())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{name}({args})")
            }
            _ => {
                let args = args
                    .iter()
                    .map(|f| f.into_python())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({args})")
            }
        }
    }
}

impl IntoPython for Operation {
    fn into_python(&self) -> String {
        self.to_string()
    }
}

impl IntoPython for Statement {
    fn into_python(&self) -> String {
        let Self {
            left_hand,
            op,
            right_hand,
        } = self;
        if left_hand.is_static && left_hand.is_final {
            return format!(
                "{} {} {}",
                left_hand.name.replace(to_constant),
                op.into_python(),
                right_hand.into_python()
            );
        }
        format!(
            "{} {} {}",
            left_hand.into_python(),
            op.into_python(),
            right_hand.into_python()
        )
    }
}

impl IntoPython for DataStruct {
    fn into_python(&self) -> String {
        let Self {
            name,
            fields,
            base_ident,
            ..
        } = self;
        let mut lines = CodeLines::new(py_file_ident(*base_ident), TERMINATOR);
        lines.push_without_terminator("@dataclass(frozen=True)");
        lines.push_without_terminator(format!("class {name}:"));
        if fields.is_empty() {
            lines.push_without_terminator("\tpass");
        } else {
            for field in fields {
                lines.push_without_terminator(format!(
                    "\t{}: {}",
                    python_ident(&field.name),
                    match &field.r#type {
                        Types::String | Types::RefString => "str".to_string(),
                        other => other.into_python(),
                    }
                ));
            }
        }
        lines.generate()
    }
}

impl IntoPython for I18nSwitch {
    fn into_python(&self) -> String {
        let Self {
            arg_name_i18n_key,
            entries,
            base_ident,
        } = self;
        let mut lines = CodeLines::new(py_file_ident(*base_ident), TERMINATOR);
        let key_arg = python_ident(arg_name_i18n_key);
        let mut first = true;
        for (key, value) in entries {
            let const_name = Naming::new(key.replace(".", "_")).replace(to_constant);
            let keyword = if first {
                first = false;
                "if"
            } else {
                "elif"
            };
            lines.push_without_terminator(format!(
                "{keyword} {key_arg} == {CONSTANTS_NAME_KEY}.{const_name}:"
            ));
            lines.push_without_terminator(format!(
                "\treturn {}",
                py_string_literal(&value.replace(python_matcher))
            ));
        }
        lines.push_without_terminator("return None");
        lines.generate()
    }
}

impl IntoPython for LanguageMatcher {
    fn into_python(&self) -> String {
        let Self {
            language_to_fn_match_entries,
            base_ident,
            arg_name_iso639,
        } = self;
        let mut lines = CodeLines::new(py_file_ident(*base_ident), TERMINATOR);
        for (lang, call) in language_to_fn_match_entries {
            lines.push_without_terminator(format!(
                "if {} == \"{lang}\":",
                python_ident(arg_name_iso639)
            ));
            lines.push_without_terminator(format!("\treturn {}", call.into_python()));
        }
        lines.generate()
    }
}

impl IntoPython for MethodCall {
    fn into_python(&self) -> String {
        let Self {
            instance,
            raw_instance_name,
            method_name,
            args_name,
            raw_args_name,
        } = self;
        let instance_name = instance
            .clone()
            .map(|name| python_ident(&name))
            .map(|name| format!("{name}."));
        let raw_instance_name = raw_instance_name.clone().map(|name| format!("{name}."));
        let args_str = args_name
            .iter()
            .map(python_ident)
            .collect::<Vec<_>>()
            .join(", ");
        let raw_args_name = raw_args_name.join(", ");
        let args_name = if raw_args_name.is_empty() {
            args_str
        } else {
            raw_args_name
        };
        let prefix = raw_instance_name.or(instance_name).unwrap_or_default();
        format!("{prefix}{}({args_name})", python_ident(method_name))
    }
}

impl IntoPython for I18nMethodContent {
    fn into_python(&self) -> String {
        let Self { tp, base_ident } = self;
        let mut lines = CodeLines::new(py_file_ident(*base_ident), TERMINATOR);
        match tp {
            I18nMethodType::Translate { language_matcher } => {
                lines
                    .extend(language_matcher.into_python())
                    .push_without_terminator("return None");
            }
            I18nMethodType::TranslateWithArgs {
                arg_name_iso639,
                arg_name_i18n_key,
                arg_name_args,
                fn_name_translate,
            } => {
                let template = format!("{}_template", python_ident(arg_name_i18n_key));
                lines
                    .push_without_terminator(format!(
                        "{template} = {}({}, {})",
                        python_ident(fn_name_translate),
                        python_ident(arg_name_iso639),
                        python_ident(arg_name_i18n_key)
                    ))
                    .push_without_terminator(format!("if not {template}:"))
                    .push_without_terminator("\treturn None")
                    .push_without_terminator(format!(
                        "return {template}.format(*{})",
                        python_ident(arg_name_args)
                    ));
            }
            I18nMethodType::MatchEntries { match_entries } => {
                lines.extend(match_entries.into_python());
            }
            I18nMethodType::Empty => (),
        }
        lines.generate()
    }
}

impl IntoPython for MethodDefinition {
    fn into_python(&self) -> String {
        let Self {
            name,
            args,
            return_type,
            ..
        } = self;
        let args = args
            .iter()
            .map(IntoPython::into_python)
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "def {}({args}) -> {}:",
            python_ident(name),
            return_type.into_python()
        )
    }
}

impl IntoPython for Method {
    fn into_python(&self) -> String {
        let Self {
            definition,
            code,
            base_ident,
        } = self;
        let mut lines = CodeLines::new(py_file_ident(*base_ident), TERMINATOR);
        lines.push_without_terminator(definition.into_python());
        let body = code.into_python();
        if body.trim().is_empty() {
            lines.push_without_terminator("\tpass");
        } else {
            lines.extend(body);
        }
        lines.generate()
    }
}

impl IntoPython for I18nCodeGenerator {
    fn into_python(&self) -> String {
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
                .join(".");
            lines.push_without_terminator(format!("# package: {path}"));
            lines.next_line_if_true(true);
        }

        lines.push_without_terminator("from __future__ import annotations");
        lines.next_line_if_true(true);
        lines.push_without_terminator("from dataclasses import dataclass");
        lines.next_line_if_true(true);

        lines
            .extend(struct_support_language_list.into_python())
            .next_line_if_true(true);

        lines.push_without_terminator("def list_support_language() -> list[SupportLanguage]:");
        lines.push_without_terminator("\treturn [");
        for InsertSupportLanguageListCall {
            language_iso639_code,
            iso639_entry,
        } in call_insert_support_language_list.iter()
        {
            lines.push_without_terminator(format!(
                "\t\tSupportLanguage(iso639_code={}, cn_name={}, en_name={}, native_name={}),",
                py_string_literal(language_iso639_code),
                py_string_literal(&iso639_entry.cn_name),
                py_string_literal(&iso639_entry.en_name),
                py_string_literal(&iso639_entry.native_name),
            ));
        }
        lines.push_without_terminator("\t]");
        lines.next_line_if_true(true);

        lines.push_without_terminator("class Iso639Codes:");
        for statement in const_support_languages {
            let name = statement.left_hand.name.replace(to_constant);
            let value = match &statement.right_hand {
                RightHand::Value(v) => v.clone(),
                RightHand::Construct(_) => "\"\"".to_string(),
            };
            lines.push_without_terminator(format!("\t{name} = {value}"));
        }
        lines.next_line_if_true(true);

        lines.push_without_terminator(format!("class {CONSTANTS_NAME_KEY}:"));
        for statement in const_i18n_keys {
            let name = statement.left_hand.name.replace(to_constant);
            let value = match &statement.right_hand {
                RightHand::Value(v) => v.clone(),
                RightHand::Construct(_) => "\"\"".to_string(),
            };
            lines.push_without_terminator(format!("\t{name} = {value}"));
        }
        lines.next_line_if_true(true);

        if let Some(fn_translate_with_args) = fn_translate_with_args {
            lines
                .extend(fn_translate_with_args.into_python())
                .next_line_if_true(true);
        }

        if let Some(fn_translate) = fn_translate {
            lines
                .extend(fn_translate.into_python())
                .next_line_if_true(true);
        }

        for (_, method) in fn_match_entries {
            lines.extend(method.into_python()).next_line_if_true(true);
        }

        lines.generate()
    }
}
