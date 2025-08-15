use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
    io::Read,
    path::PathBuf,
};

use anyhow::{Context, Result};
use serde_json::{Map, Value};
use toml::Table;
use whatlang::{Detector, Lang};

use crate::{
    common::{i18n_entry::I18nEntry, iso639_codes::whatlang_to_iso639},
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

pub struct I18nFileReader<W: FnMut(&str, &str, &str)> {
    pub src_dir: PathBuf,
    pub writer: W,
}

impl<W: FnMut(&str, &str, &str)> I18nFileReader<W> {
    pub fn run(&mut self) -> Result<()> {
        let mut i18n_files = HashSet::new();
        Self::read_files(self.src_dir.clone(), &mut i18n_files)?;

        let detector = Detector::with_denylist(vec![Lang::Nob, Lang::Pes]);

        for file in i18n_files {
            let mut lang = get_language_hint_from_filename(&file.get_path()).ok();
            let entries = match file {
                InputFile::Json(path_buf) => read_i18n_entries_from_json(path_buf)?,
                InputFile::Yaml(path_buf) => todo!(),
                InputFile::Toml(path_buf) => read_i18n_entries_from_toml(path_buf)?,
                InputFile::Properties(path_buf) => todo!(),
            };
            lang = lang.or(detect_language(&entries, &detector));
            let iso639_code = match lang {
                Some(iso639_code) => iso639_code,
                None => continue,
            };
            for entry in entries {
                (self.writer)(iso639_code, &entry.key, &entry.value.i18n_content);
            }
        }
        Ok(())
    }

    fn read_files(current_dir: PathBuf, i18n_files: &mut HashSet<InputFile>) -> Result<()> {
        if !current_dir.exists() {
            return Ok(());
        }
        if current_dir.is_file() {
            if let Some(file) = Self::select_target_file(current_dir) {
                i18n_files.insert(file);
            }
            return Ok(());
        }
        for dir in current_dir.read_dir()? {
            let dir = dir?;
            let next_path = dir.path();
            Self::read_files(next_path, i18n_files)?;
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

fn get_language_hint_from_filename(file_path: &PathBuf) -> Result<&'static str> {
    let file_name = file_path
        .file_name()
        .context("Failed to get file name")?
        .to_string_lossy()
        .to_string()
        .to_lowercase()
        .replace("-", "_");
    if file_name.contains("_zh") || file_name.contains("_cn") {
        return Ok("zh");
    } else if file_name.contains("_en") || file_name.contains("_us") {
        return Ok("en");
    } else if file_name.contains("_ja") || file_name.contains("_jp") {
        return Ok("ja");
    } else if file_name.contains("_ar") {
        return Ok("ar");
    } else if file_name.contains("_es") {
        return Ok("es");
    }
    Err(anyhow::anyhow!("Failed to detect language from file name"))
}

fn detect_language(entries: &HashSet<I18nEntry>, detector: &Detector) -> Option<&'static str> {
    let mut codes = entries
        .iter()
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

    let maybe_code = codes.last_entry().map(|ent| whatlang_to_iso639(ent.get()));
    maybe_code
}

fn read_i18n_entries_from_toml(file_path: PathBuf) -> Result<HashSet<I18nEntry>> {
    let mut file = std::fs::OpenOptions::new().read(true).open(&file_path)?;
    let mut entries_str = String::new();

    fn fold_table(prev_key: &str, table: &toml::Table) -> Vec<(String, String)> {
        let entries = table
            .iter()
            .filter_map(|(key, value)| {
                let new_key = if !prev_key.is_empty() {
                    format!("{prev_key}.{key}")
                } else {
                    key.to_string()
                };
                if let Some(table) = value.as_table() {
                    let entries = fold_table(&new_key, table);
                    Some(entries)
                } else if let Some(value) = value.as_str() {
                    Some(vec![(new_key, value.to_string())])
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        entries
    }

    file.read_to_string(&mut entries_str)?;
    let toml_table = toml::from_str::<Table>(&entries_str)?;
    let entries = fold_table("", &toml_table)
        .into_iter()
        .map(|(key, value)| I18nEntry {
            key,
            value: FormatContent {
                i18n_content: value.to_string(),
            },
        })
        .collect::<HashSet<_>>();

    Ok(entries)
}

fn read_i18n_entries_from_json(file_path: PathBuf) -> Result<HashSet<I18nEntry>> {
    let mut file = std::fs::OpenOptions::new().read(true).open(&file_path)?;
    let mut entries_str = String::new();
    file.read_to_string(&mut entries_str)?;

    fn fold_json_map(prev_key: &str, object: &Map<String, Value>) -> Vec<(String, String)> {
        let entries = object
            .iter()
            .filter_map(|(key, value)| {
                let new_key = if !prev_key.is_empty() {
                    format!("{prev_key}.{key}")
                } else {
                    key.to_string()
                };
                if let Some(table) = value.as_object() {
                    let entries = fold_json_map(&new_key, table);
                    Some(entries)
                } else if let Some(value) = value.as_str() {
                    Some(vec![(new_key, value.to_string())])
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        entries
    }

    let mut entries = Default::default();
    let json_entries = serde_json::from_str::<Value>(&entries_str)?;
    if let Some(object) = json_entries.as_object() {
        entries = fold_json_map("", object)
            .into_iter()
            .map(|(key, value)| I18nEntry {
                key,
                value: FormatContent {
                    i18n_content: value.to_string(),
                },
            })
            .collect::<HashSet<_>>();
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v = read_i18n_entries_from_toml(
            format!("C:\\Users\\machi\\workspace\\tools\\ri18n\\tests\\data\\i18n_en.toml").into(),
        )
        .unwrap();
        println!("{v:#?}");
    }
}
