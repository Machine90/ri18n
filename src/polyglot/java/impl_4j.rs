use crate::polyglot::{
    common::{
        code_lines::CodeLines,
        data_struct::DataStruct,
        field::Field,
        i18n_code_generator::{I18nCodeGenerator, InsertSupportLanguageListCall},
        i18n_enum::I18nEnum,
        i18n_method::{I18nMethodContent, I18nMethodType, Method},
        iso639_codes::iso639_map,
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
            r#type,
            name,
        } = self;
        let modifier = match (is_public, is_static) {
            (true, true) => format!("public static "),
            (true, false) => format!("public "),
            (false, true) => format!("static "),
            (false, false) => format!(""),
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
            I18nMethodType::BuildI18nKey {
                arg_name_iso639,
                arg_name_i18n_key,
            } => {
                lines
                    .push_without_terminator(format!(
                        "if ({arg_name_iso639} == null || {arg_name_i18n_key} == null || {arg_name_i18n_key}.isEmpty()) {{"
                    ))
                    .push(format!("\treturn null"))
                    .push_without_terminator(format!("}}"))
                    .push(format!("return String.format(\"%s.%s\", {arg_name_iso639}.code, {arg_name_i18n_key})"));
            }
            I18nMethodType::GetByLocale {
                method_name_build_i18n_key,
                arg_name_iso639,
                arg_name_i18n_key,
                instance_name_language_map,
            } => {
                let full_key = format!("{}{}", arg_name_i18n_key, "Full");
                lines
                    .push(format!(
                        "String {full_key} = {method_name_build_i18n_key}({arg_name_iso639}, {arg_name_i18n_key})"
                    ))
                    .push(format!("return {instance_name_language_map}.get({full_key})"));
            }
            I18nMethodType::GetByLocaleWithArgs {
                method_name_build_i18n_key,
                arg_name_iso639,
                arg_name_i18n_key,
                instance_name_language_map,
            } => {
                let full_key = format!("{}{}", arg_name_i18n_key, "Full");
                let template = format!("{}{}", arg_name_i18n_key, "Template");
                lines
                    .push(format!(
                        "String {full_key} = {method_name_build_i18n_key}({arg_name_iso639}, {arg_name_i18n_key})"
                    ))
                    .push(format!("String {template} = {instance_name_language_map}.get({full_key})"))
                    .push_without_terminator(format!("if ({template} == null || {template}.isEmpty()) {{"))
                    .push(format!("\treturn null"))
                    .push_without_terminator(format!("}}"))
                    .push(format!("return {template}.formatted(args)"));
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
            name,
            fields,
            base_ident,
        } = self;
        let mut fields = fields.clone();
        fields.sort_by(|f1, f2| f1.name.cmp(&f2.name));
        let mut lines = CodeLines::new(*base_ident, TERMINATOR);
        lines.push_without_terminator(format!("public static class {name} {{"));
        for field in fields {
            lines.push(format!("\t{}", field.into_java()));
        }

        lines
            .extend(self.into_constructor())
            .next_line_if_true(true)
            .push_without_terminator("}".to_string())
            .generate()
    }
}

impl IntoJavaConstructor for DataStruct {
    fn into_constructor(&self) -> String {
        let Self {
            name,
            fields,
            base_ident,
        } = self;
        let mut lines = CodeLines::new(*base_ident + 1, TERMINATOR);
        let mut fields = fields.clone();
        fields.sort_by(|f1, f2| f1.name.cmp(&f2.name));
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

impl IntoJava for I18nEnum {
    fn into_java(&self) -> String {
        if self.language_iso639_codes.is_empty() {
            return format!("");
        }
        let enum_name = self.name.as_str();
        let mut lines = CodeLines::new(self.base_ident, None);
        lines.push(format!("public static enum {enum_name} {{"));
        for iso639_code in self.language_iso639_codes.iter() {
            let name = iso639_code.to_uppercase();
            let value = iso639_code.to_lowercase();
            if !iso639_map().contains_key(value.as_str()) {
                panic!("Invalid language code");
            }
            lines.push(format!("\t{name}(\"{value}\"),"));
        }
        lines.push("\t;".to_string());
        lines.next_line_if_true(true);

        let field = Field {
            r#type: Types::String,
            name: "code".to_string(),
            ..Default::default()
        };
        lines.push(format!("\t{}", field.into_java()));
        lines.next_line_if_true(true);

        let data_struct = DataStruct {
            name: enum_name.to_string(),
            fields: vec![field],
            base_ident: self.base_ident + 1,
        }
        .into_java();
        lines.extend(data_struct).next_line_if_true(true);
        lines.push("}".to_string()).generate()
    }
}

impl IntoJava for I18nCodeGenerator {
    fn into_java(&self) -> String {
        let Self {
            package,
            imports,
            name,
            struct_support_language_list,
            fn_build_i18n_key: build_i18n_key_method,
            fn_get_by_locale: get_by_locale_method,
            fn_get_by_locale_with_args: get_by_locale_method_with_args,
            stat_build_support_language_list: build_support_language_list,
            call_insert_support_language_list: insert_support_language_list,
            stat_build_i18n_map: build_i18n_map,
            call_insert_i18n_map,
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
            .extend(build_i18n_key_method.into_java())
            .next_line_if_true(true);

        lines
            .extend(get_by_locale_method.into_java())
            .next_line_if_true(true);

        lines
            .extend(get_by_locale_method_with_args.into_java())
            .next_line_if_true(true);

        lines.push(format!("\t{}", build_support_language_list.into_java()));

        // add support language list
        let instance = build_support_language_list.left_hand.name.as_str();
        lines.push_without_terminator("\tstatic {".to_string());
        for InsertSupportLanguageListCall {
            language_iso639_code,
            iso639_entry,
        } in insert_support_language_list.iter()
        {
            let constructed = struct_support_language_list.construct_java(vec![
                &format!("\"{}\"", iso639_entry.cn_name),
                &format!("\"{}\"", iso639_entry.en_name),
                &format!("\"{language_iso639_code}\""),
                &format!("\"{}\"", iso639_entry.native_name),
            ]);
            lines.push(format!("\t\t{instance}.add({constructed})"));
        }
        lines
            .push_without_terminator("\t}".to_string())
            .next_line_if_true(true);

        // put i18n map
        lines.push(format!("\t{}", build_i18n_map.into_java()));

        lines.push_without_terminator("}".to_owned());
        lines.generate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::polyglot::common::{data_struct::DataStruct, used_types::Types};

    #[test]
    fn test_java_types() {
        let string = Types::String.into_java();
        assert_eq!(string, "String");
        let list = Types::List(Box::new(Types::String)).into_java();
        assert_eq!(list, "java.util.ArrayList<String>");
        let map =
            Types::HashMap(Box::new(Types::String), Box::new(Types::Custom("Integer"))).into_java();
        assert_eq!(map, "java.util.HashMap<String, Integer>");
    }

    #[test]
    fn test_java_field() {
        let field = Field::public(Types::List(Box::new(Types::String)), "name").into_java();
        assert_eq!(field, "public java.util.ArrayList<String> name");
    }

    #[test]
    fn test_java_statement() {
        let tp = Types::String;
        let stat = Statement {
            left_hand: Field::private(tp.clone(), "name"),
            op: Operation::Assign,
            right_hand: RightHand::Value("\"This is name.\"".to_string()),
        };
        let code = stat.into_java();
        println!("{code}");
    }

    #[test]
    fn test_java_method() {
        // Method {
        //     name: "Test".to_string(),
        //     args: vec![Field::private(Types::String, "code")],
        //     is_public: true,
        //     is_static: true,
        //     codes: vec![],
        //     ..Default::default()
        // };
    }

    #[test]
    fn test() {
        let language_map = Field::new(
            true,
            true,
            Types::HashMap(Box::new(Types::String), Box::new(Types::String)),
            "map",
        );
        let build_i18n_key_method = Method::new_method_build_i18n_key(None, None, None, 1);
        let mut i18n_gen = I18nCodeGenerator {
            package: None,
            imports: Default::default(),
            name: "Test".to_string(),
            struct_support_language_list: DataStruct {
                name: "SupportLanguage".to_string(),
                base_ident: 1,
                fields: vec![
                    Field::public(Types::String, "iso639_code"),
                    Field::public(Types::String, "cn_name"),
                    Field::public(Types::String, "en_name"),
                    Field::public(Types::String, "native_name"),
                ],
            },
            fn_get_by_locale: Method::new_method_get_by_locale(
                None,
                &build_i18n_key_method.name,
                language_map.name.as_str(),
                None,
                None,
                1,
            ),
            fn_get_by_locale_with_args: Method::new_method_get_by_locale(
                None,
                &build_i18n_key_method.name,
                language_map.name.as_str(),
                None,
                None,
                1,
            ),
            fn_build_i18n_key: build_i18n_key_method,
            stat_build_support_language_list: Statement {
                left_hand: Field::new(
                    true,
                    true,
                    Types::List(Box::new(Types::Custom("SupportLanguage"))),
                    "list",
                ),
                op: Operation::Assign,
                right_hand: RightHand::Construct(Construct {
                    tp: Types::List(Box::new(Types::Custom("SupportLanguage"))),
                    args: vec![],
                }),
            },
            call_insert_support_language_list: Default::default(),
            stat_build_i18n_map: Statement {
                left_hand: language_map,
                op: Operation::Assign,
                right_hand: RightHand::Construct(Construct {
                    tp: Types::HashMap(Box::new(Types::String), Box::new(Types::String)),
                    args: vec![],
                }),
            },
            call_insert_i18n_map: Default::default(),
        };
        i18n_gen
            .insert_i18n_entry("en", "test", "test")
            .insert_i18n_entry("zh", "test", "测试")
            .insert_i18n_entry("ja", "test", "测试");
        let code = i18n_gen.into_java();
        println!("{code}");
    }
}
