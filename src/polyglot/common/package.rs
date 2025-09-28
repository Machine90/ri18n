use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Result, anyhow};

#[derive(Debug, Default, Clone)]
pub struct Package {
    pub package_path: PathBuf,
}

impl FromStr for Package {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let package_path = if s.contains(".") {
            s.split(".")
                .map(|seg| Path::new(seg.trim()).to_path_buf())
                .collect::<PathBuf>()
        } else if s.contains("/") {
            s.split("/")
                .map(|seg| Path::new(seg.trim()).to_path_buf())
                .collect::<PathBuf>()
        } else if s.contains("::") {
            s.split("::")
                .map(|seg| Path::new(seg.trim()).to_path_buf())
                .collect::<PathBuf>()
        } else {
            Path::new(s.trim()).to_path_buf()
        };
        Ok(Self { package_path })
    }
}

#[derive(Debug, Clone)]
pub enum PackageAction {
    Import(Package),
    Define(Package),
}
