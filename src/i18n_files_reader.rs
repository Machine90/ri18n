use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use hashlink::LinkedHashMap;
use serde_json::{Map, Value};
use toml::Table;
use whatlang::{Detector, Lang};
use yaml_rust2::{Yaml, yaml};

use crate::{
    common::{
        i18n_entry::I18nEntry,
        iso639_codes::{iso639_map, whatlang_to_iso639},
    },
    polyglot::common::FormatContent,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum InputFile {
    Json(PathBuf),
    Yaml(PathBuf),
    Toml(PathBuf),
    Properties(PathBuf),
}

impl InputFile {
    fn get_path(&self) -> PathBuf {
        match self {
            InputFile::Json(path_buf) => path_buf,
            InputFile::Yaml(path_buf) => path_buf,
            InputFile::Toml(path_buf) => path_buf,
            InputFile::Properties(path_buf) => path_buf,
        }
        .clone()
    }
}

/// The i18n file reader will detect all supported i18n file under the `src_dir`,
/// then read all entries from each file, the key of i18n entries will be converted
/// to `x.y.z` format.
pub struct I18nFileReader<W: FnMut(&str, &str, &str)> {
    pub src_dir: PathBuf,
    pub writer: W,
}

impl<W: FnMut(&str, &str, &str)> I18nFileReader<W> {
    pub fn run(&mut self) -> Result<()> {
        if !self.src_dir.exists() {
            bail!(
                "i18n source path does not exist: {}",
                self.src_dir.display()
            );
        }

        let mut i18n_files = HashSet::new();
        Self::read_files(self.src_dir.clone(), &mut i18n_files)?;
        if i18n_files.is_empty() {
            bail!(
                "no supported i18n files found under {} (expected .json/.yaml/.yml/.toml/.properties)",
                self.src_dir.display()
            );
        }

        let detector = Detector::with_denylist(vec![Lang::Nob, Lang::Pes]);
        let mut loaded_entries = 0usize;
        let mut skipped_undetectable = 0usize;

        for file in i18n_files {
            let path = file.get_path();
            let mut lang = get_language_hint_from_filename(&path).ok();
            let entries = match file {
                InputFile::Json(path_buf) => read_i18n_entries_from_json(path_buf)
                    .with_context(|| format!("failed to read json i18n file {}", path.display()))?,
                InputFile::Yaml(path_buf) => read_i18n_entries_from_yaml(path_buf)
                    .with_context(|| format!("failed to read yaml i18n file {}", path.display()))?,
                InputFile::Toml(path_buf) => read_i18n_entries_from_toml(path_buf)
                    .with_context(|| format!("failed to read toml i18n file {}", path.display()))?,
                InputFile::Properties(path_buf) => read_i18n_entries_from_properties(path_buf)
                    .with_context(|| {
                        format!("failed to read properties i18n file {}", path.display())
                    })?,
            };
            if entries.is_empty() {
                tracing::warn!(
                    path = %path.display(),
                    "skipping i18n file with no entries"
                );
                continue;
            }
            lang = lang.or_else(|| detect_language(entries.values(), &detector));
            let iso639_code = match lang {
                Some(iso639_code) => iso639_code,
                None => {
                    skipped_undetectable += 1;
                    tracing::warn!(
                        path = %path.display(),
                        "skipping i18n file: cannot detect language from filename or content; rename like i18n_<iso639>.{{json,yaml,toml,properties}}"
                    );
                    continue;
                }
            };
            loaded_entries += entries.len();
            for entry in entries.values() {
                (self.writer)(&iso639_code, &entry.key, &entry.value.i18n_content);
            }
        }

        if loaded_entries == 0 {
            bail!(
                "no i18n entries were loaded from {} ({} file(s) skipped because language was undetectable)",
                self.src_dir.display(),
                skipped_undetectable
            );
        }
        Ok(())
    }

    fn read_files(current_dir: PathBuf, i18n_files: &mut HashSet<InputFile>) -> Result<()> {
        if current_dir.is_file() {
            if let Some(file) = Self::select_target_file(current_dir) {
                i18n_files.insert(file);
            }
            return Ok(());
        }
        for dir in current_dir
            .read_dir()
            .with_context(|| format!("failed to read directory {}", current_dir.display()))?
        {
            let dir = dir.with_context(|| {
                format!(
                    "failed to read directory entry under {}",
                    current_dir.display()
                )
            })?;
            Self::read_files(dir.path(), i18n_files)?;
        }
        Ok(())
    }

    fn select_target_file(file: PathBuf) -> Option<InputFile> {
        if let Some(ext) = file.extension() {
            let ext = ext.to_string_lossy().to_string();
            let file = match ext.as_str() {
                "json" => InputFile::Json(file),
                "yaml" | "yml" => InputFile::Yaml(file),
                "toml" => InputFile::Toml(file),
                "properties" => InputFile::Properties(file),
                _ => return None,
            };
            return Some(file);
        }
        None
    }
}

fn get_language_hint_from_filename(file_path: &PathBuf) -> Result<String> {
    let file_name = file_path
        .file_name()
        .context("Failed to get file name")?
        .to_string_lossy()
        .to_string()
        .to_lowercase()
        .replace("-", "_");
    let file_name = file_name
        .split(".")
        .next()
        .context("Failed to get file name without extension")?;
    let iso639_code = if file_name.contains('_') {
        file_name
            .split('_')
            .last()
            .context("failed to parse language hint from filename")?
    } else {
        file_name
    }
    .to_lowercase();
    if !iso639_map().contains_key(iso639_code.as_str()) {
        bail!("invalid iso639 code in filename: {iso639_code}");
    }
    Ok(iso639_code)
}

fn detect_language<'a, I>(entries: I, detector: &Detector) -> Option<String>
where
    I: IntoIterator<Item = &'a I18nEntry>,
{
    let mut codes = entries
        .into_iter()
        .filter_map(|ent| detector.detect(&ent.value.i18n_content))
        .fold(HashMap::<Lang, (u64, u32)>::new(), |mut map, info| {
            let confidence = (info.confidence() * 1_000_000_000.) as u64;
            let lang = info.lang();
            let (_, count) = map.entry(lang).or_insert((confidence, 0));
            *count = *count + 1;
            map
        })
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect::<BTreeMap<(u64, u32), Lang>>();

    codes
        .last_entry()
        .map(|ent| whatlang_to_iso639(ent.get()).to_string())
}

fn entries_from_pairs(
    path: &Path,
    pairs: Vec<(String, String)>,
) -> Result<BTreeMap<String, I18nEntry>> {
    let mut map: BTreeMap<String, I18nEntry> = BTreeMap::new();
    for (key, value) in pairs {
        if let Some(prev) = map.get(&key) {
            if prev.value.i18n_content != value {
                bail!(
                    "duplicate i18n key `{key}` with conflicting values in {}",
                    path.display()
                );
            }
            continue;
        }
        map.insert(
            key.clone(),
            I18nEntry {
                key,
                value: FormatContent {
                    i18n_content: value,
                },
            },
        );
    }
    Ok(map)
}

fn read_i18n_entries_from_toml(file_path: PathBuf) -> Result<BTreeMap<String, I18nEntry>> {
    let mut file = std::fs::OpenOptions::new().read(true).open(&file_path)?;
    let mut entries_str = String::new();

    fn fold_table(prev_key: &str, table: &toml::Table) -> Vec<(String, String)> {
        table
            .iter()
            .filter_map(|(key, value)| {
                let new_key = if !prev_key.is_empty() {
                    format!("{prev_key}.{key}")
                } else {
                    key.to_string()
                };
                if let Some(table) = value.as_table() {
                    Some(fold_table(&new_key, table))
                } else if let Some(value) = value.as_str() {
                    Some(vec![(new_key, value.to_string())])
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    file.read_to_string(&mut entries_str)?;
    let toml_table = toml::from_str::<Table>(&entries_str)?;
    entries_from_pairs(&file_path, fold_table("", &toml_table))
}

fn read_i18n_entries_from_json(file_path: PathBuf) -> Result<BTreeMap<String, I18nEntry>> {
    let mut file = std::fs::OpenOptions::new().read(true).open(&file_path)?;
    let mut entries_str = String::new();
    file.read_to_string(&mut entries_str)?;

    fn fold_json_map(prev_key: &str, object: &Map<String, Value>) -> Vec<(String, String)> {
        object
            .iter()
            .filter_map(|(key, value)| {
                let new_key = if !prev_key.is_empty() {
                    format!("{prev_key}.{key}")
                } else {
                    key.to_string()
                };
                if let Some(table) = value.as_object() {
                    Some(fold_json_map(&new_key, table))
                } else if let Some(value) = value.as_str() {
                    Some(vec![(new_key, value.to_string())])
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    let json_entries = serde_json::from_str::<Value>(&entries_str)?;
    let pairs = json_entries
        .as_object()
        .map(|object| fold_json_map("", object))
        .unwrap_or_default();
    entries_from_pairs(&file_path, pairs)
}

fn read_i18n_entries_from_properties(file_path: PathBuf) -> Result<BTreeMap<String, I18nEntry>> {
    let file = std::fs::OpenOptions::new().read(true).open(&file_path)?;
    let map = java_properties::read(file)?;
    let pairs = map.into_iter().collect::<Vec<_>>();
    entries_from_pairs(&file_path, pairs)
}

fn read_i18n_entries_from_yaml(file_path: PathBuf) -> Result<BTreeMap<String, I18nEntry>> {
    let file = std::fs::OpenOptions::new().read(true).open(&file_path)?;
    let mut f = yaml::YamlDecoder::read(file);

    fn fold_yaml_map(prev_key: &str, object: &LinkedHashMap<Yaml, Yaml>) -> Vec<(String, String)> {
        object
            .iter()
            .filter_map(|(key, value)| {
                let new_key = if !prev_key.is_empty() {
                    let key = key.as_str()?;
                    format!("{prev_key}.{key}")
                } else {
                    key.as_str()?.to_string()
                };
                if let Some(table) = value.as_hash() {
                    Some(fold_yaml_map(&new_key, table))
                } else if let Some(value) = value.as_str() {
                    Some(vec![(new_key, value.to_string())])
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    let yaml = f
        .decode()
        .map_err(|e| anyhow::anyhow!("failed to decode yaml file {file_path:?}: {e:?}"))?;
    let pairs = yaml
        .into_iter()
        .filter_map(|v| {
            let v = v.as_hash()?;
            Some(fold_yaml_map("", v))
        })
        .flatten()
        .collect::<Vec<_>>();
    entries_from_pairs(&file_path, pairs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use rstest::rstest;

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data/i18n")
            .join(name)
    }

    #[rstest]
    #[test_log::test]
    #[case::de("i18n_DE.properties", true, Some("de"))]
    #[case::en("i18n_en.yaml", true, Some("en"))]
    #[case::unknown("i18n_unknown.toml", false, None)]
    fn test_language_hint_from_filename(
        #[case] file: &str,
        #[case] expect_ok: bool,
        #[case] expected: Option<&str>,
    ) {
        let result = get_language_hint_from_filename(&fixture(file));
        assert_eq!(result.is_ok(), expect_ok);
        if let Some(expected) = expected {
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[rstest]
    #[test_log::test]
    fn test_run_fails_when_language_undetectable_for_all_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("messages.toml");
        std::fs::write(
            &path,
            r#"
greeting = "12345"
count = "999"
"#,
        )
        .expect("write fixture");

        let mut loaded = 0usize;
        let mut reader = I18nFileReader {
            src_dir: dir.path().to_path_buf(),
            writer: |_, _, _| loaded += 1,
        };
        let err = reader
            .run()
            .expect_err("undetectable language should fail the run when nothing loads");
        assert!(err.to_string().contains("no i18n entries were loaded"));
        assert_eq!(loaded, 0);
    }

    #[rstest]
    #[test_log::test]
    fn test_duplicate_key_with_conflict_fails() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("i18n_en.toml");
        // Same dotted key reachable twice with different values via nesting vs flat
        // is hard in toml; use properties-style duplicates via a crafted fold input
        // by writing invalid-for-unique json object... JSON can't duplicate keys.
        // Use entries_from_pairs directly.
        let err = entries_from_pairs(
            path.as_path(),
            vec![("a.b".into(), "one".into()), ("a.b".into(), "two".into())],
        )
        .expect_err("conflicting duplicate key should fail");
        assert!(err.to_string().contains("duplicate i18n key"));
    }

    #[rstest]
    #[test_log::test]
    fn test_duplicate_key_same_value_ok() {
        let path = PathBuf::from("virtual.toml");
        let map = entries_from_pairs(
            &path,
            vec![("a.b".into(), "same".into()), ("a.b".into(), "same".into())],
        )
        .expect("identical duplicates should be accepted");
        assert_eq!(map.len(), 1);
        assert_eq!(map["a.b"].value.i18n_content, "same");
    }
}
