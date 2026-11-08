use std::str::FromStr;

use ri18n::polyglot::common::package::Package;
use rstest::rstest;

#[rstest]
#[test_log::test]
#[case::rust_path("com::a::b", &["com", "a", "b"])]
#[case::mixed_should_prefer_colons("a::b.c", &["a", "b.c"])]
#[case::slash("com/a/b", &["com", "a", "b"])]
#[case::dot("com.a.b", &["com", "a", "b"])]
fn test_package_from_str(#[case] input: &str, #[case] expected: &[&str]) {
    let package = Package::from_str(input).expect("parse package");
    let parts: Vec<_> = package
        .package_path
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();
    assert_eq!(parts, expected);
}
