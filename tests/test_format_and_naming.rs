use ri18n::polyglot::common::{
    FormatContent, Naming, java_matcher, python_matcher, rust_matcher, to_camelcase, to_constant,
    to_snake_case,
};
use rstest::rstest;

#[rstest]
#[test_log::test]
#[case::legacy(
    "User name is {str} age is {u32} weight is {f32}",
    "User name is %s age is %d weight is %f"
)]
#[case::aliases(
    "name={string}/{s}/{text} age={int}/{integer}/{long} weight={float}/{double} ok={boolean} ch={char}",
    "name=%s/%s/%s age=%d/%d/%d weight=%f/%f ok=%b ch=%c"
)]
#[case::rust_ints("a={i8} b={isize} c={byte} d={short}", "a=%d b=%d c=%d d=%d")]
#[case::unknown_preserved("keep {unknown_token} and {foo}", "keep {unknown_token} and {foo}")]
fn test_format_convert_java(#[case] input: &str, #[case] expected: &str) {
    let fmt = FormatContent {
        i18n_content: input.to_string(),
    };
    assert_eq!(fmt.replace(java_matcher), expected);
}

#[rstest]
#[test_log::test]
#[case::aliases(
    "name={text} age={integer} weight={double} ok={boolean}",
    "name={} age={} weight={} ok={}"
)]
fn test_format_convert_rust(#[case] input: &str, #[case] expected: &str) {
    let fmt = FormatContent {
        i18n_content: input.to_string(),
    };
    assert_eq!(fmt.replace(rust_matcher), expected);
}

#[rstest]
#[test_log::test]
#[case::aliases(
    "name={text} age={integer} weight={double} ok={boolean}",
    "name={} age={} weight={} ok={}"
)]
fn test_format_convert_python(#[case] input: &str, #[case] expected: &str) {
    let fmt = FormatContent {
        i18n_content: input.to_string(),
    };
    assert_eq!(fmt.replace(python_matcher), expected);
}

#[rstest]
#[test_log::test]
#[case::camel("this_is_snake_case", "thisIsSnakeCase")]
fn test_to_camelcase(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(Naming::new(input).replace(to_camelcase), expected);
}

#[rstest]
#[test_log::test]
#[case::snake("thisIs_Camel_case", "this_is_camel_case")]
fn test_to_snake_case(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(Naming::new(input).replace(to_snake_case), expected);
}

#[rstest]
#[test_log::test]
#[case::constant("thisIs_Camel_case", "THIS_IS_CAMEL_CASE")]
fn test_to_constant(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(Naming::new(input).replace(to_constant), expected);
}
