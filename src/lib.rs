use std::{
    collections::{BTreeSet, HashMap},
    path::Path,
    sync::Arc,
};

use crate::{
    args::Target,
    common::constants::*,
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
        java::JavaCodeGen,
    },
};

pub mod args;
pub mod common;
pub mod i18n_files_reader;
pub mod polyglot;

pub trait IntoCode: 'static {
    fn into_code(&self) -> String;
}

pub struct I18nCodeGen {
    generators: HashMap<Target, Box<dyn IntoCode>>,
}

impl I18nCodeGen {
    pub fn new<P: AsRef<Path>>(package: Option<Package>, name: String, i18n_asset_dir: P) -> Self {
        let src_dir = i18n_asset_dir.as_ref().to_path_buf();

        let language_list = Field {
            is_public: true,
            is_static: true,
            is_final: true,
            r#type: Types::List(Box::new(Types::Custom(STRUCT_NAME_SUPPORT_LANGUAGE))),
            name: Naming::new(INSTANCE_NAME_LANGUAGE_LIST),
        };

        let mut i18n_gen = I18nCodeGenerator {
            package,
            name,
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
            fn_get_by_locale_with_args: Method::new_method_get_by_locale_with_args(
                None, None, None, 1,
            ),
            fn_match_entries: Default::default(),
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
        let mut i18n_files_reader = I18nFileReader {
            src_dir,
            writer: |iso639_code, i18n_key, value| {
                i18n_gen.insert_i18n_entry(iso639_code, i18n_key, value);
            },
        };
        if let Err(e) = i18n_files_reader.run() {
            eprintln!("Failed to read file, error: {e:?}");
        }
        let generator = Arc::new(i18n_gen);

        let mut generators = HashMap::new();
        // Add Java target
        let java_codegen: Box<dyn IntoCode> = Box::new(JavaCodeGen {
            generator: generator.clone(),
        });
        generators.insert(Target::Java, java_codegen);
        Self { generators }
    }

    pub fn generate_code(&self, target: Target) -> Option<String> {
        self.generators.get(&target).map(|g| g.into_code())
    }
}
