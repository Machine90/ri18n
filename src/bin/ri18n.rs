use std::{collections::BTreeSet, str::FromStr};

use clap::Parser;

use ri18n::{
    args::{Args, Target},
    common::constants::{
        ARG_NAME_ISO639, FIELD_NAME_CN_NAME, FIELD_NAME_EN_NAME, FIELD_NAME_NATIVE_NAME,
        INSTANCE_NAME_LANGUAGE_LIST, STRUCT_NAME_SUPPORT_LANGUAGE,
    },
    i18n_files_reader::I18nFileReader,
    polyglot::{
        common::{
            Naming,
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

fn main() {
    let args = Args::parse();

    let language_list = Field {
        is_public: true,
        is_static: true,
        is_final: true,
        r#type: Types::List(Box::new(Types::Custom(STRUCT_NAME_SUPPORT_LANGUAGE))),
        name: Naming::new(INSTANCE_NAME_LANGUAGE_LIST),
    };
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
            name: STRUCT_NAME_SUPPORT_LANGUAGE.to_string(),
            base_ident: 1,
            fields: BTreeSet::from_iter([
                Field::public(Types::String, ARG_NAME_ISO639),
                Field::public(Types::String, FIELD_NAME_CN_NAME),
                Field::public(Types::String, FIELD_NAME_EN_NAME),
                Field::public(Types::String, FIELD_NAME_NATIVE_NAME),
            ]),
            with_constructor: true,
            ..Default::default()
        },
        fn_get_by_locale: Method::new_method_get_by_locale(None, None, 1),
        fn_get_by_locale_with_args: Method::new_method_get_by_locale_with_args(None, None, None, 1),
        const_support_languages: [].into(),
        stat_build_support_language_list: Statement {
            left_hand: language_list,
            op: Operation::Assign,
            right_hand: RightHand::Construct(Construct {
                tp: Types::List(Box::new(Types::Custom(STRUCT_NAME_SUPPORT_LANGUAGE))),
                args: vec![],
            }),
        },
        call_insert_support_language_list: Default::default(),
        const_i18n_keys: [].into(),
    };

    let src_dir = args.src_dir.to_path_buf();
    let mut i18n_files_reader = I18nFileReader {
        src_dir,
        writer: |iso639_code, i18n_key, value| {
            i18n_gen.insert_i18n_entry(iso639_code, i18n_key, value);
        },
    };
    if let Err(e) = i18n_files_reader.run() {
        eprintln!("Failed to read file, error: {e:?}");
    }

    let (file_name, code) = match args.language {
        Target::Java => (format!("{}.java", args.name), i18n_gen.into_java()),
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
