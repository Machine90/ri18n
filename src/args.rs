use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand, Clone, Debug)]
#[clap(rename_all = "kebab-case")]
pub enum Target {
    Java,
}

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, env)]
    pub i18n_dir: PathBuf,
    #[command(subcommand)]
    pub target_language: Target,
    #[arg(long, env)]
    pub target_basedir: PathBuf,
    #[arg(long, env)]
    pub target_package: Option<String>,
    #[arg(long, env)]
    pub target_name: String,
}
