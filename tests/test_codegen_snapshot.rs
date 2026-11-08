mod common;

use std::str::FromStr;

use ri18n::{I18nCodeGen, args::Target, polyglot::common::package::Package};
use rstest::rstest;

use common::{assert_matches_golden, fixture_dir};

fn generate(target: Target, package: Option<&str>, name: &str, constants_only: bool) -> String {
    let package = package.map(|p| Package::from_str(p).expect("package"));
    let codegen = I18nCodeGen::new(package, name.to_string(), fixture_dir(), constants_only)
        .expect("load i18n fixtures");
    codegen
        .generate_code(target)
        .expect("generate target language code")
}

#[rstest]
#[test_log::test]
#[case::java_full(
    Target::Java,
    Some("com.example.ri18n"),
    "Ri18n",
    false,
    "Ri18n.java",
    true
)]
#[case::java_constants_only(
    Target::Java,
    Some("com.example.ri18n"),
    "Ri18nConstantsOnly",
    true,
    "Ri18nConstantsOnly.java",
    false
)]
#[case::rust_full(
    Target::Rust {
        formatx_enable: false
    },
    None,
    "ri18n",
    false,
    "ri18n.rs",
    true
)]
#[case::rust_formatx(
    Target::Rust {
        formatx_enable: true
    },
    None,
    "ri18n",
    false,
    "ri18n_formatx.rs",
    true
)]
#[case::rust_constants_only(
    Target::Rust {
        formatx_enable: false
    },
    None,
    "ri18n",
    true,
    "ri18n_constants_only.rs",
    false
)]
#[case::typescript_full(
    Target::Typescript,
    Some("com.example.ri18n"),
    "ri18n",
    false,
    "ri18n.ts",
    true
)]
#[case::typescript_constants_only(
    Target::Typescript,
    None,
    "ri18nConstantsOnly",
    true,
    "ri18nConstantsOnly.ts",
    false
)]
#[case::python_full(
    Target::Python,
    Some("com.example.ri18n"),
    "ri18n",
    false,
    "ri18n.py",
    true
)]
#[case::python_constants_only(
    Target::Python,
    None,
    "ri18n_constants_only",
    true,
    "ri18n_constants_only.py",
    false
)]
fn test_codegen_matches_golden(
    #[case] target: Target,
    #[case] package: Option<&str>,
    #[case] name: &str,
    #[case] constants_only: bool,
    #[case] golden: &str,
    #[case] expect_lookup_api: bool,
) {
    let code = generate(target.clone(), package, name, constants_only);
    assert_matches_golden(golden, &code);

    match target {
        Target::Java => {
            assert!(code.contains("Iso639Codes"));
            assert!(code.contains("Keys"));
            assert_eq!(code.contains("translate("), expect_lookup_api);
            // With-args must use a distinct name (no overload collision).
            assert_eq!(code.contains("translateWithArgs("), expect_lookup_api);
            if golden == "Ri18nConstantsOnly.java" {
                assert!(code.contains("public class Ri18nConstantsOnly"));
            }
        }
        Target::Rust { formatx_enable } => {
            assert!(code.contains("mod keys"));
            assert!(code.contains("mod iso639_codes"));
            assert_eq!(code.contains("fn translate"), expect_lookup_api);
            if formatx_enable {
                assert!(code.contains("macro_rules! translate_with_args"));
                assert!(code.contains("formatx::formatx"));
            } else if expect_lookup_api {
                assert!(!code.contains("macro_rules! translate_with_args"));
            }
        }
        Target::Typescript => {
            assert!(code.contains("export const Iso639Codes"));
            assert!(code.contains("export const Keys"));
            assert_eq!(code.contains("function translate("), expect_lookup_api);
            assert_eq!(
                code.contains("function translateWithArgs("),
                expect_lookup_api
            );
        }
        Target::Python => {
            assert!(code.contains("class Iso639Codes:"));
            assert!(code.contains("class Keys:"));
            assert_eq!(code.contains("def translate("), expect_lookup_api);
            assert_eq!(code.contains("def translate_with_args("), expect_lookup_api);
        }
    }
}
