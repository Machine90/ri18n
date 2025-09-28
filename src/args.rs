use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand, Clone, Debug, PartialEq, Eq, Hash)]
#[clap(rename_all = "kebab-case")]
pub enum Target {
    Java,
}

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, env)]
    pub src_dir: PathBuf,
    #[arg(short, long, env, default_value = "./")]
    pub target_dir: PathBuf,
    #[arg(short, long, env)]
    pub package: Option<String>,
    #[arg(short, long, env, default_value = "Ri18n")]
    pub name: String,
    /// Generate constant codes only if true
    #[arg(short, long, env, default_value = "false", action = ArgAction::SetTrue)]
    pub constants_only: bool,
    #[command(subcommand)]
    pub language: Target,
}
