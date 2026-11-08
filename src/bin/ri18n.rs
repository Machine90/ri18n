use std::str::FromStr;

use anyhow::{Context, Result, bail};
use clap::Parser;

use ri18n::{
    I18nCodeGen,
    args::{Args, Target},
    polyglot::common::package::Package,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    let package = args
        .package
        .as_deref()
        .map(Package::from_str)
        .transpose()
        .context("invalid package, format should be 'com.a.b', 'com/a/b', or 'com::a::b'")?;

    let code_gen = I18nCodeGen::new(
        package.clone(),
        args.name.clone(),
        &args.src_dir,
        args.constants_only,
    )?;
    let code = code_gen.generate_code(args.language.clone())?;

    let file_name = match args.language {
        Target::Java => format!("{}.java", args.name),
        Target::Rust { .. } => format!("{}.rs", args.name),
    };
    let mut save_to = args.target_dir;
    if save_to.is_file() {
        bail!(
            "`target_dir` must be a directory, got file {}",
            save_to.display()
        );
    }
    if let Some(package) = package {
        save_to = save_to.join(package.package_path);
    }
    if !save_to.exists() {
        std::fs::create_dir_all(&save_to)
            .with_context(|| format!("failed to create output directory {}", save_to.display()))?;
    }
    let save_to_file = save_to.join(file_name);
    std::fs::write(&save_to_file, code).with_context(|| {
        format!(
            "failed to write generated code to {}",
            save_to_file.display()
        )
    })?;
    Ok(())
}
