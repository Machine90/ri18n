mod common;

use std::str::FromStr;

use ri18n::{I18nCodeGen, args::Target, polyglot::common::package::Package};
use rstest::rstest;

use common::{assert_matches_golden, fixture_dir};

fn generate(target: Target, package: Option<&str>, constants_only: bool) -> String {
    let package = package.map(|p| Package::from_str(p).expect("package"));
    let codegen = I18nCodeGen::new(package, "Ri18n".to_string(), fixture_dir(), constants_only);
    codegen
        .generate_code(target)
        .expect("generate target language code")
}

#[rstest]
#[test_log::test]
#[case::java_full(Target::Java, Some("com.example.ri18n"), false, "Ri18n.java", true)]
#[case::java_constants_only(
    Target::Java,
    Some("com.example.ri18n"),
    true,
    "Ri18n_constants_only.java",
    false
)]
#[case::rust_full(
    Target::Rust {
        formatx_enable: false
    },
    None,
    false,
    "ri18n.rs",
    true
)]
#[case::rust_formatx(
    Target::Rust {
        formatx_enable: true
    },
    None,
    false,
    "ri18n_formatx.rs",
    true
)]
#[case::rust_constants_only(
    Target::Rust {
        formatx_enable: false
    },
    None,
    true,
    "ri18n_constants_only.rs",
    false
)]
fn test_codegen_matches_golden(
    #[case] target: Target,
    #[case] package: Option<&str>,
    #[case] constants_only: bool,
    #[case] golden: &str,
    #[case] expect_lookup_api: bool,
) {
    let code = generate(target.clone(), package, constants_only);
    assert_matches_golden(golden, &code);

    match target {
        Target::Java => {
            assert!(code.contains("Iso639Codes"));
            assert!(code.contains("Keys"));
            assert_eq!(code.contains("getByLocale"), expect_lookup_api);
        }
        Target::Rust { formatx_enable } => {
            assert!(code.contains("mod keys"));
            assert!(code.contains("mod iso639_codes"));
            if formatx_enable {
                assert!(code.contains("macro_rules! get_by_locale"));
                assert!(code.contains("formatx::formatx"));
            } else {
                assert_eq!(code.contains("fn get_by_locale"), expect_lookup_api);
            }
        }
    }
}
