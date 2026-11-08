use std::{
    collections::{BTreeSet, HashMap},
    path::Path,
    sync::Arc,
};

use anyhow::{Context, Result, bail};

use crate::{
    args::{Target, TargetKind},
    common::constants::*,
    i18n_files_reader::I18nFileReader,
    polyglot::{
        common::{
            Naming,
            data_struct::DataStruct,
            field::Field,
            i18n_code_generator::I18nCodeGenerator,
            i18n_method::Method,
            package::{Package, PackageAction},
            statement::{Operation, RightHand, Statement},
            used_types::{Construct, Types},
        },
        java::JavaCodeGen,
        rust::RustCodeGen,
        typescript::TypeScriptCodeGen,
    },
};

pub mod args;
pub mod common;
pub mod i18n_files_reader;
pub mod polyglot;

pub trait IntoCode: 'static {
    fn into_code(&self, target: Target) -> String;
}

pub struct I18nCodeGen {
    generators: HashMap<TargetKind, Box<dyn IntoCode>>,
}

impl I18nCodeGen {
    pub fn new<P: AsRef<Path>>(
        package: Option<Package>,
        name: String,
        i18n_asset_dir: P,
        generate_constants_only: bool,
    ) -> Result<Self> {
        let src_dir = i18n_asset_dir.as_ref().to_path_buf();

        let language_list = Field {
            is_public: true,
            is_static: true,
            is_final: true,
            r#type: Types::List(Box::new(Types::Custom(
                STRUCT_NAME_SUPPORT_LANGUAGE.to_string(),
            ))),
            name: Naming::new(INSTANCE_NAME_LANGUAGE_LIST),
        };

        let mut fn_translate = None;
        let mut fn_translate_with_args = None;
        if !generate_constants_only {
            fn_translate = Some(Method::new_method_translate(None, None, 1));
            fn_translate_with_args =
                Some(Method::new_method_translate_with_args(None, None, None, 1));
        }

        let mut i18n_gen = I18nCodeGenerator {
            package: package.map(PackageAction::Define),
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
            fn_translate,
            fn_translate_with_args,
            fn_match_entries: Default::default(),
            const_support_languages: [].into(),
            stat_build_support_language_list: Statement {
                left_hand: language_list,
                op: Operation::Assign,
                right_hand: RightHand::Construct(Construct {
                    tp: Types::List(Box::new(Types::Custom(
                        STRUCT_NAME_SUPPORT_LANGUAGE.to_string(),
                    ))),
                    args: vec![],
                }),
            },
            call_insert_support_language_list: Default::default(),
            const_i18n_keys: [].into(),
            keys_by_locale: Default::default(),
        };
        let mut i18n_files_reader = I18nFileReader {
            src_dir: src_dir.clone(),
            writer: |iso639_code, i18n_key, value| {
                i18n_gen.insert_i18n_entry(iso639_code, i18n_key, value, generate_constants_only);
            },
        };
        i18n_files_reader
            .run()
            .with_context(|| format!("failed to load i18n files from {}", src_dir.display()))?;

        if i18n_gen.call_insert_support_language_list.is_empty() {
            bail!(
                "no support languages were produced from {}",
                src_dir.display()
            );
        }
        i18n_gen.warn_locale_key_mismatches();

        let generator = Arc::new(i18n_gen);

        let mut generators = HashMap::new();
        let java_codegen: Box<dyn IntoCode> = Box::new(JavaCodeGen {
            generator: generator.clone(),
        });
        generators.insert(TargetKind::Java, java_codegen);
        let rust_codegen: Box<dyn IntoCode> = Box::new(RustCodeGen {
            generator: generator.clone(),
        });
        generators.insert(TargetKind::Rust, rust_codegen);
        let typescript_codegen: Box<dyn IntoCode> = Box::new(TypeScriptCodeGen {
            generator: generator.clone(),
        });
        generators.insert(TargetKind::Typescript, typescript_codegen);
        Ok(Self { generators })
    }

    pub fn generate_code(&self, target: Target) -> Result<String> {
        let kind = match &target {
            Target::Java => TargetKind::Java,
            Target::Rust { .. } => TargetKind::Rust,
            Target::Typescript => TargetKind::Typescript,
        };
        self.generators
            .get(&kind)
            .map(|g| g.into_code(target))
            .ok_or_else(|| anyhow::anyhow!("no code generator registered for target {kind:?}"))
    }
}
