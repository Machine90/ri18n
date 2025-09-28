use clap::{ArgAction, Parser, Subcommand};
use enum_kinds::EnumKind;
use std::path::PathBuf;

#[derive(Subcommand, Clone, Debug, EnumKind, PartialEq, Eq, Hash)]
#[clap(rename_all = "kebab-case")]
#[enum_kind(TargetKind, derive(Hash))]
pub enum Target {
    Java,
    Rust {
        /// If enable this value, code generator will using [formatx](https://crates.io/crates/formatx)
        /// to format i18n content with arguments.
        ///
        /// ## Requirements
        /// Then you should add `formatx` dependency to your project:
        /// ```toml
        /// [dependencies]
        /// formatx = "0.2"
        /// ```
        #[arg(short, long, env, default_value = "false", action = ArgAction::SetTrue)]
        formatx_enable: bool,
    },
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
