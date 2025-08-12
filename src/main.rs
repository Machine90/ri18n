use std::str::FromStr;

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

    let language_map = Field::new(
        true,
        true,
        Types::HashMap(Box::new(Types::String), Box::new(Types::String)),
        "map",
    );
    let build_i18n_key_method = Method::new_method_build_i18n_key(None, None, None, 1);
    let package = args
        .target_package
        .map(|p| Package::from_str(&p))
        .transpose()
        .expect("Invalid package, format should be 'com.a.b' or 'com::a::b'");
    let mut i18n_gen = I18nCodeGenerator {
        package: package.clone(),
        imports: Default::default(),
        name: args.target_name,
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

    let code = match args.target_language {
        args::Target::Java => i18n_gen.into_java(),
    };
    let save_to = args.target_basedir;
}
