#![allow(dead_code)]

use std::{fs, path::PathBuf};

pub fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/i18n")
}

pub fn fixture_file(name: &str) -> PathBuf {
    fixture_dir().join(name)
}

pub fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden")
        .join(name)
}

pub fn assert_matches_golden(name: &str, actual: &str) {
    let path = golden_path(name);
    if std::env::var("UPDATE_GOLDEN").is_ok() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create golden dir");
        }
        fs::write(&path, actual).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
        return;
    }

    let expected = fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "missing golden file {} ({e}); regenerate with UPDATE_GOLDEN=1 cargo nextest run",
            path.display()
        )
    });
    assert_eq!(
        expected,
        actual,
        "generated code diverged from {}; regenerate with UPDATE_GOLDEN=1 cargo nextest run if intentional",
        path.display()
    );
}
