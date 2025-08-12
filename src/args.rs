use std::path::PathBuf;

pub enum Target {
    Java,
}

pub struct Args {
    pub i18n_dir: PathBuf,
    pub target_language: Target,
    pub target_basedir: PathBuf,
    pub target_package: Option<String>,
    pub target_name: String,
}

impl Args {
    pub fn new() -> Self {
        todo!()
    }
}
