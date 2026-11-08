use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Result;

#[derive(Debug, Default, Clone)]
pub struct Package {
    pub package_path: PathBuf,
}

impl FromStr for Package {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Prefer the most specific separator first so `a::b.c` is not split on `.`.
        let package_path = if s.contains("::") {
            s.split("::")
                .map(|seg| Path::new(seg.trim()).to_path_buf())
                .collect::<PathBuf>()
        } else if s.contains('/') {
            s.split('/')
                .map(|seg| Path::new(seg.trim()).to_path_buf())
                .collect::<PathBuf>()
        } else if s.contains('.') {
            s.split('.')
                .map(|seg| Path::new(seg.trim()).to_path_buf())
                .collect::<PathBuf>()
        } else {
            Path::new(s.trim()).to_path_buf()
        };
        Ok(Self { package_path })
    }
}

/// Language-neutral namespace path action.
/// - Java: `Define` → `package ...;`, `Import` → `import ...;`
/// - Rust: both resolve to a `::`-joined module path; `Define` is not emitted as a file header.
#[derive(Debug, Clone)]
pub enum PackageAction {
    Import(Package),
    Define(Package),
}

impl PackageAction {
    pub fn package(&self) -> &Package {
        match self {
            PackageAction::Import(package) | PackageAction::Define(package) => package,
        }
    }
}
