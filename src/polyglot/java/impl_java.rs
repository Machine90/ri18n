use crate::common::code_lines::CodeLines;
use crate::common::i18n_entry::I18nEntry;
use crate::polyglot::common::i18n_switch::{I18nSwitch, LanguageMatcher};
use crate::polyglot::{
    common::{
        data_struct::DataStruct,
        field::Field,
        i18n_code_generator::{I18nCodeGenerator, InsertSupportLanguageListCall},
        i18n_method::{I18nMethodContent, I18nMethodType, Method, MethodCall},
        java_matcher,
        package::Package,
        statement::{Operation, RightHand, Statement},
        used_types::{Construct, Types},
    },
    java::{IntoJava, IntoJavaConstruct, IntoJavaConstructor},
};
const TERMINATOR: Option<&'static str> = Some(";");

impl IntoJava for Package {
    fn into_java(&self) -> String {
        let package = self
            .package_path
            .into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(".");
        format!("import {package}")
    }
}

impl IntoJava for Types {
    fn into_java(&self) -> String {
        match self {
            Types::String => "String".to_string(),
            Types::List(generic_type) => {
                format!("java.util.ArrayList<{}>", generic_type.into_java())
            }
            Types::HashMap(generic_key, generic_value) => {
                format!(
                    "java.util.HashMap<{}, {}>",
                    generic_key.into_java(),
                    generic_value.into_java()
                )
            }
            Types::Custom(tp) => tp.to_string(),
            Types::Void => "void".to_string(),
            Types::ListAny => "Object...".to_string(),
        }
    }
}

impl IntoJava for Construct {
    fn into_java(&self) -> String {
        let Self { tp, args } = self;
        let type_str = tp.into_java();
        let initial_value = args
            .iter()
            .map(|f| f.into_java())
            .collect::<Vec<_>>()
            .join(", ");
        format!("new {type_str}({initial_value})")
    }
}

impl IntoJava for Field {
    fn into_java(&self) -> String {
        let Self {
            is_public,
            is_static,
            is_final,
            r#type,
            name,
        } = self;
        let modifier = match (is_public, is_static, is_final) {
            (true, true, true) => format!("public static final "),
            (true, true, false) => format!("public static "),
            (true, false, true) => format!("public "),
            (true, false, false) => format!("public "),
            (false, true, true) => format!("static final "),
            (false, true, false) => format!("static "),
            (false, false, true) => format!(""),
            (false, false, false) => format!(""),
        };
        format!("{modifier}{} {name}", r#type.into_java())
    }
}

impl IntoJava for RightHand {
    fn into_java(&self) -> String {
        match self {
            RightHand::Value(v) => v.to_string(),
            RightHand::Construct(construct) => construct.into_java(),
        }
    }
}

impl IntoJava for Operation {
    fn into_java(&self) -> String {
        self.to_string()
    }
}

impl IntoJava for Statement {
    fn into_java(&self) -> String {
        let Self {
            left_hand,
            op,
            right_hand,
        } = self;
        format!(
            "{} {} {}",
            left_hand.into_java(),
            op.into_java(),
            right_hand.into_java()
        )
    }
}

impl IntoJava for I18nMethodContent {
    fn into_java(&self) -> String {
        let Self { tp, base_ident } = self;
        let mut lines = CodeLines::new(*base_ident, None);
        match tp {
            I18nMethodType::GetByLocale { language_matcher } => {
                lines
                    .extend(language_matcher.into_java())
                    .push(format!("return null"));
            }
            I18nMethodType::GetByLocaleWithArgs {
                arg_name_iso639,
                arg_name_i18n_key,
                arg_name_args,
                fn_name_get_locale,
            } => {
                let template = format!("{}{}", arg_name_i18n_key, "Template");
                lines
                    .push(format!(
                        "String {template} = {fn_name_get_locale}({arg_name_iso639}, {arg_name_i18n_key})"
                    ))
                    .push_without_terminator(format!(
                        "if ({template} == null || {template}.isEmpty()) {{"
                    ))
                    .push(format!("\treturn null"))
                    .push_without_terminator(format!("}}"))
                    .push(format!("return {template}.formatted({arg_name_args})"));
            }
            I18nMethodType::Empty => (),
        };
        lines.generate()
    }
}

impl IntoJavaConstruct for DataStruct {
    fn construct_java(&self, args: Vec<&str>) -> String {
        let Self { name, .. } = self;
        let arg_list = args.join(", ");
        format!("new {name}({arg_list})")
    }
}

impl IntoJava for DataStruct {
    fn into_java(&self) -> String {
        let Self {
            is_static,
            name,
            fields,
            statements,
            with_constructor,
            base_ident,
        } = self;
        let mut lines = CodeLines::new(*base_ident, TERMINATOR);

        let modifier = match *is_static {
            true => "static ",
            false => "",
        };
        lines.push_without_terminator(format!("public {modifier}class {name} {{"));
        for field in fields.iter() {
            lines.push(format!("\t{}", field.into_java()));
        }
        lines.next_line_if_true(!fields.is_empty());
        for stat in statements.iter() {
            lines.push(format!("\t{}", stat.into_java()));
        }
        lines.next_line_if_true(!statements.is_empty());

        if *with_constructor {
            lines
                .extend(self.into_constructor())
                .next_line_if_true(true);
        }

        lines.push_without_terminator("}".to_string()).generate()
    }
}

impl IntoJavaConstructor for DataStruct {
    fn into_constructor(&self) -> String {
        let Self {
            name,
            fields,
            base_ident,
            ..
        } = self;
        let mut lines = CodeLines::new(*base_ident + 1, TERMINATOR);
        let args = fields
            .iter()
            .map(|f| format!("{} {}", f.r#type.into_java(), f.name))
            .collect::<Vec<_>>()
            .join(", ");
        lines.push_without_terminator(format!("{name}({args}) {{"));
        for f in fields {
            let field_name = f.name.as_str();
            lines.push(format!("\tthis.{field_name} = {field_name}"));
        }
        lines.push_without_terminator("}".to_string()).generate()
    }
}

impl IntoJava for I18nSwitch {
    fn into_java(&self) -> String {
        let Self {
            arg_name_i18n_key,
            entries,
            base_ident,
        } = self;

        let mut lines = CodeLines::new(*base_ident, TERMINATOR);
        lines.push_without_terminator(format!("switch ({arg_name_i18n_key}) {{"));
        for I18nEntry { key, value } in entries {
            lines
                .push_without_terminator(format!("\tcase \"{key}\":"))
                .push(format!("\t\treturn \"{}\"", value.replace(java_matcher)));
        }
        lines.push_without_terminator("}".to_string()).generate()
    }
}

impl IntoJava for LanguageMatcher {
    fn into_java(&self) -> String {
        let Self {
            language_to_entries,
            base_ident,
            arg_name_iso639,
            ..
        } = self;

        let mut lines = CodeLines::new(*base_ident, TERMINATOR);
        for (lang, switch) in language_to_entries {
            lines.push_without_terminator(format!("if ({arg_name_iso639} == \"{lang}\") {{"));
            lines.extend(switch.into_java());
            lines.push_without_terminator("}".to_string());
        }
        lines.generate()
    }
}

impl IntoJava for Method {
    fn into_java(&self) -> String {
        let Self {
            name,
            args,
            return_type,
            is_public,
            is_static,
            code,
            base_ident,
        } = self;
        let modifier = match (is_public, is_static) {
            (true, true) => format!("public static "),
            (true, false) => format!("public "),
            (false, true) => format!("static "),
            (false, false) => format!(""),
        };
        let args = args
            .iter()
            .map(IntoJava::into_java)
            .collect::<Vec<_>>()
            .join(", ");
        let mut lines = CodeLines::new(*base_ident, TERMINATOR);
        lines.push_without_terminator(format!(
            "{modifier}{} {name}({args}) {{",
            return_type.into_java()
        ));
        lines.extend(code.into_java());
        lines.push_without_terminator("}".to_string());
        lines.generate()
    }
}

impl IntoJava for MethodCall {
    fn into_java(&self) -> String {
        let Self {
            instance,
            method_name,
            args,
        } = self;
        let instance_name = instance
            .clone()
            .map(|i| format!("{i}."))
            .unwrap_or_default();
        let args_str = args.join(", ");
        format!("{instance_name}{method_name}({args_str})")
    }
}

impl IntoJava for I18nCodeGenerator {
    fn into_java(&self) -> String {
        let Self {
            package,
            name,
            struct_support_language_list,
            fn_get_by_locale,
            fn_get_by_locale_with_args,
            const_support_languages,
            stat_build_support_language_list,
            call_insert_support_language_list,
            const_i18n_keys,
        } = self;
        let mut lines = CodeLines::new(0, TERMINATOR);
        if let Some(pkg) = package {
            lines.push(pkg.into_java());
        }
        lines.next_line_if_true(package.is_some());
        lines
            .push_without_terminator(format!("public class {name} {{"))
            .next_line_if_true(true);

        // support language struct
        lines
            .extend(struct_support_language_list.into_java())
            .next_line_if_true(true);

        lines
            .extend(fn_get_by_locale_with_args.into_java())
            .next_line_if_true(true);

        lines.push(format!(
            "\t{}",
            stat_build_support_language_list.into_java()
        ));

        // render iso639 codes constants
        let mut iso639_class = DataStruct {
            is_static: true,
            name: "Iso639Codes".to_string(),
            base_ident: 1,
            ..Default::default()
        };
        for statement in const_support_languages {
            iso639_class.statements.insert(statement.clone());
        }
        lines
            .extend(iso639_class.into_java())
            .next_line_if_true(true);

        // add support language list
        let instance_support_language = stat_build_support_language_list.left_hand.name.as_str();
        lines.push_without_terminator("\tstatic {".to_string());
        for InsertSupportLanguageListCall {
            language_iso639_code,
            iso639_entry,
        } in call_insert_support_language_list.iter()
        {
            let constructed = struct_support_language_list.construct_java(vec![
                &format!("\"{}\"", iso639_entry.cn_name),
                &format!("\"{}\"", iso639_entry.en_name),
                &format!("\"{language_iso639_code}\""),
                &format!("\"{}\"", iso639_entry.native_name),
            ]);
            let call_add_support_language = MethodCall {
                instance: Some(instance_support_language.to_string()),
                method_name: "add".to_string(),
                args: vec![constructed],
                ..Default::default()
            };
            lines.push(format!("\t\t{}", call_add_support_language.into_java()));
        }
        lines
            .push_without_terminator("\t}".to_string())
            .next_line_if_true(true);

        // render keys constants
        let mut keys_class = DataStruct {
            is_static: true,
            name: "Keys".to_string(),
            base_ident: 1,
            ..Default::default()
        };
        for const_key in const_i18n_keys {
            keys_class.statements.insert(const_key.clone());
        }
        lines.extend(keys_class.into_java()).next_line_if_true(true);

        lines
            .extend(fn_get_by_locale.into_java())
            .next_line_if_true(true);

        lines.push_without_terminator("}".to_owned());
        lines.generate()
    }
}
