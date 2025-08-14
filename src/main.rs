use std::{collections::BTreeSet, str::FromStr};

use clap::Parser;

use crate::{
    args::Args,
    polyglot::{
        common::{
            data_struct::DataStruct,
            field::Field,
            i18n_code_generator::I18nCodeGenerator,
            i18n_method::Method,
            package::Package,
            statement::{Operation, RightHand, Statement},
            used_types::{Construct, Types},
        },
        java::IntoJava,
    },
};

pub mod args;
pub mod polyglot;

fn main() {
    let args = Args::parse();

    let language_map = Field {
        is_public: true,
        is_static: true,
        is_final: true,
        r#type: Types::HashMap(Box::new(Types::String), Box::new(Types::String)),
        name: "map".to_string(),
    };
    let language_list = Field {
        is_public: true,
        is_static: true,
        is_final: true,
        r#type: Types::List(Box::new(Types::Custom("SupportLanguage"))),
        name: "list".to_string(),
    };
    let build_i18n_key_method = Method::new_method_build_i18n_key(None, None, None, 1);
    let package = args
        .package
        .map(|p| Package::from_str(&p))
        .transpose()
        .expect("Invalid package, format should be 'com.a.b' or 'com::a::b'");
    let mut i18n_gen = I18nCodeGenerator {
        package: package.clone(),
        name: args.name.clone(),
        struct_support_language_list: DataStruct {
            is_static: true,
            name: "SupportLanguage".to_string(),
            base_ident: 1,
            fields: BTreeSet::from_iter([
                Field::public(Types::String, "iso639_code"),
                Field::public(Types::String, "cn_name"),
                Field::public(Types::String, "en_name"),
                Field::public(Types::String, "native_name"),
            ]),
            with_constructor: true,
            ..Default::default()
        },
        fn_get_by_locale: Method::new_method_get_by_locale(
            None,
            &build_i18n_key_method.name,
            language_map.name.as_str(),
            None,
            None,
            1,
        ),
        fn_get_by_locale_with_args: Method::new_method_get_by_locale_with_args(
            None,
            &build_i18n_key_method.name,
            language_map.name.as_str(),
            None,
            None,
            None,
            1,
        ),
        fn_build_i18n_key: build_i18n_key_method,
        const_support_languages: [].into(),
        stat_build_support_language_list: Statement {
            left_hand: language_list,
            op: Operation::Assign,
            right_hand: RightHand::Construct(Construct {
                tp: Types::List(Box::new(Types::Custom("SupportLanguage"))),
                args: vec![],
            }),
        },
        call_insert_support_language_list: Default::default(),
        const_i18n_keys: [].into(),
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

    let (file_name, code) = match args.language {
        args::Target::Java => (format!("{}.java", args.name), i18n_gen.into_java()),
    };
    let mut save_to = args.target_dir;
    if save_to.is_file() {
        panic!("`target_dir` must be dir");
    }
    if let Some(package) = package {
        save_to = save_to.join(package.package_path);
    }
    if !save_to.exists() {
        std::fs::create_dir_all(&save_to).expect("Failed to create dirs");
    }
    let save_to_file = save_to.join(file_name);
    std::fs::write(save_to_file, code).expect("Failed to generate code");
}
