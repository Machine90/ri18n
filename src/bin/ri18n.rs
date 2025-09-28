use std::str::FromStr;

use clap::Parser;

use ri18n::{
    I18nCodeGen,
    args::{Args, Target},
    polyglot::common::package::Package,
};

fn main() {
    let args = Args::parse();
    let package = args
        .package
        .map(|p| Package::from_str(&p))
        .transpose()
        .expect("Invalid package, format should be 'com.a.b' or 'com::a::b'");
    let code_gen = I18nCodeGen::new(
        package.clone(),
        args.name.clone(),
        args.src_dir,
        args.constants_only,
    );
    let code = code_gen
        .generate_code(args.language)
        .expect("Failed to generate target language code");

    let file_name = match args.language {
        Target::Java => format!("{}.java", args.name),
    };
    let mut save_to = args.target_dir;
    if save_to.is_file() {
        panic!("`target_dir` must be dir");
    }
    if let Some(package) = package {
        save_to = save_to.join(package.package_path);
    }
    if !save_to.exists() {
        std::fs::create_dir_all(&save_to).expect("Failed to create dirs");
    }
    let save_to_file = save_to.join(file_name);
    std::fs::write(save_to_file, code).expect("Failed to generate code");
}
