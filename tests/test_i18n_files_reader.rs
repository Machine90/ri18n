mod common;

use std::collections::HashSet;

use ri18n::i18n_files_reader::I18nFileReader;
use rstest::rstest;

use common::{fixture_dir, fixture_file};

fn collect_entries(src: std::path::PathBuf) -> Vec<(String, String, String)> {
    let mut entries = Vec::new();
    let mut reader = I18nFileReader {
        src_dir: src,
        writer: |iso639, key, value| {
            entries.push((iso639.to_string(), key.to_string(), value.to_string()));
        },
    };
    reader.run().expect("failed to read i18n fixtures");
    entries.sort();
    entries
}

#[rstest]
#[test_log::test]
#[case::toml("i18n_zh.toml", "zh", "animal.cat.name", "老虎")]
#[case::json("i18n_ja.json", "ja", "human.man.name", "ボブ")]
#[case::yaml(
    "i18n_en.yaml",
    "en",
    "animal.cat.desc",
    "This is a tiger, his name is {s}, age is {u32} and weight is {f32}"
)]
#[case::properties("i18n_DE.properties", "de", "human.woman.name", "Alice")]
fn test_reads_entries_with_filename_locale(
    #[case] file: &str,
    #[case] expected_locale: &str,
    #[case] expected_key: &str,
    #[case] expected_value: &str,
) {
    let entries = collect_entries(fixture_file(file));
    assert!(!entries.is_empty());
    assert!(entries.iter().all(|(iso, _, _)| iso == expected_locale));
    assert!(
        entries
            .iter()
            .any(|(_, key, value)| key == expected_key && value == expected_value)
    );
}

#[rstest]
#[test_log::test]
#[case::spanish("i18n_unknown.toml", "es")]
#[case::arabic("i18n_middle_east.toml", "ar")]
fn test_detects_locale_from_content_when_filename_has_no_iso639(
    #[case] file: &str,
    #[case] expected_locale: &str,
) {
    let entries = collect_entries(fixture_file(file));
    assert!(!entries.is_empty());
    assert!(entries.iter().all(|(iso, _, _)| iso == expected_locale));
}

#[rstest]
#[test_log::test]
fn test_reads_all_fixtures_under_i18n_dir() {
    let entries = collect_entries(fixture_dir());
    let locales: HashSet<_> = entries.iter().map(|(iso, _, _)| iso.as_str()).collect();
    for expected in ["ar", "de", "en", "es", "ja", "zh"] {
        assert!(
            locales.contains(expected),
            "missing locale {expected} in {locales:?}"
        );
    }
}
