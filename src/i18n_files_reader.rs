use std::{collections::HashSet, hash::Hash, io::Read, path::PathBuf};

use anyhow::{Context, Result};
use serde_json::Value;
use toml::Table;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum InputFile {
    Json(PathBuf),
    Yaml(PathBuf),
    Toml(PathBuf),
    Properties(PathBuf),
}

#[derive(Debug, Clone)]
struct I18nEntry {
    key: String,
    value: String,
}

impl Hash for I18nEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl Eq for I18nEntry {}

impl PartialEq for I18nEntry {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

#[derive(Debug)]
struct I18nEntries {
    iso639_code: String,
    entries: HashSet<I18nEntry>,
}

pub struct I18nFileReader<W: FnMut(&str, &str, &str)> {
    pub src_dir: PathBuf,
    pub writer: W,
}

impl<W: FnMut(&str, &str, &str)> I18nFileReader<W> {
    pub fn run(&mut self) -> Result<()> {
        let mut i18n_files = HashSet::new();
        Self::read_files(self.src_dir.clone(), &mut i18n_files)?;

        for file in i18n_files {
            let result = match file {
                InputFile::Json(path_buf) => todo!(),
                InputFile::Yaml(path_buf) => todo!(),
                InputFile::Toml(path_buf) => read_i18n_entries_from_toml(path_buf)?,
                InputFile::Properties(path_buf) => todo!(),
            };
            let iso639_code = result.iso639_code.as_str();
            for entry in result.entries {
                (self.writer)(iso639_code, &entry.key, &entry.value);
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

    fn read_i18n_entries_from_json(file_path: PathBuf) -> Result<I18nEntries> {
        let mut file = std::fs::OpenOptions::new().read(true).open(&file_path)?;
        let mut entries_str = String::new();
        file.read_to_string(&mut entries_str);
        let json_entries = serde_json::from_str::<Value>(&entries_str)?;
        todo!()
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

fn read_i18n_entries_from_toml(file_path: PathBuf) -> Result<I18nEntries> {
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
        .map(|(key, value)| I18nEntry { key, value })
        .collect::<HashSet<_>>();

    let iso639_code = get_language_hint_from_filename(&file_path)?;

    Ok(I18nEntries {
        iso639_code: iso639_code.to_string(),
        entries,
    })
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
