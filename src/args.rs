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
        /// Note that after enabling this configuration, you need to add the `formatx` dependency to your
        /// project using the command `cargo add formatx`.
        #[arg(short, long, env, default_value = "false", action = ArgAction::SetTrue)]
        formatx_enable: bool,
    },
    /// Emit a TypeScript module (`.ts`) with `translate` / `translateWithArgs`.
    Typescript,
}

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, env)]
    pub src_dir: PathBuf,
    #[arg(short, long, env, default_value = "./")]
    pub target_dir: PathBuf,
    #[arg(short, long, env)]
    pub package: Option<String>,
    /// Generated file name without extension.
    #[arg(short, long, env, default_value = "Ri18n")]
    pub name: String,
    /// Generate constant codes only if true
    #[arg(short, long, env, default_value = "false", action = ArgAction::SetTrue)]
    pub constants_only: bool,
    #[command(subcommand)]
    pub language: Target,
}
