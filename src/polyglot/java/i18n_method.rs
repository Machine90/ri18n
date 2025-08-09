use crate::polyglot::{ident_lines::IdentLines, java::constants::*};

#[derive(Debug)]
pub struct Arg {
    arg_type: String,
    arg_name: String,
}

impl Arg {
    pub fn new(tp: &str, name: &str) -> Self {
        Self {
            arg_type: tp.to_string(),
            arg_name: name.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Method {
    name: String,
    args: Vec<Arg>,
    output_type: String,
    is_static: bool,
    statements: Vec<String>,
    base_ident: u8,
}

impl Method {
    pub fn new_method_build_i18n_key(base_ident: u8) -> Self {
        Self {
            name: METHOD_NAME_BUILD_I18N_KEY.to_string(),
            args: vec![
                Arg::new(ENUM_NAME_ISO_639_CODE, "iso639"),
                Arg::new(JAVA_STRING, "key"),
            ],
            output_type: "String".to_string(),
            is_static: true,
            statements: vec![
                "if (iso639 == null || key == null || key.isEmpty()) {".to_string(),
                "\treturn null;".to_string(),
                "}".to_string(),
                "return String.format(\"%s.%s\", iso639.code, key);".to_string(),
            ],
            base_ident,
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        let mut lines = IdentLines::new(self.base_ident);
        let method_name = self.name.as_str();
        let modifier = if self.is_static {
            format!("{JAVA_STATIC} ")
        } else {
            "".to_string()
        };
        let output = self.output_type.as_str();
        let args = self
            .args
            .iter()
            .map(|arg| format!("{} {}", arg.arg_type, arg.arg_name))
            .collect::<Vec<_>>()
            .join(", ");
        lines.push(format!("{modifier}{output} {method_name}({args}) {{"));
        for statement in self.statements.iter() {
            lines.push(format!("\t{}", statement.to_string()));
        }
        lines.push(format!("}}"));
        lines.generate()
    }
}
