use std::str::FromStr;

use crate::{
    common::code_lines::CodeLines,
    polyglot::{
        common::{
            i18n_code_generator::I18nCodeGenerator,
            package::{Package, PackageAction},
        },
        rust::{IntoRust, RustSettings},
    },
};

const TERMINATOR: Option<&'static str> = Some(";");

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

impl IntoRust for I18nCodeGenerator {
    fn into_rust(&self, settings: &RustSettings) -> String {
        let Self {
            struct_support_language_list,
            fn_get_by_locale_with_args,
            const_support_languages,
            stat_build_support_language_list,
            call_insert_support_language_list,
            const_i18n_keys,
            fn_get_by_locale,
            fn_match_entries,
            ..
        } = self;
        let mut use_crates = vec![];
        if settings.formatx_enable {
            use_crates.push(PackageAction::Import(
                Package::from_str("formatx::formatx").expect("Failed to parse package"),
            ));
        }

        let mut lines = CodeLines::new(0, TERMINATOR);
        for use_crate in use_crates {
            lines.push(use_crate.into_rust(settings));
        }

        lines.generate()
    }
}
