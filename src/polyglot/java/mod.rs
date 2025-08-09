use std::path::{Path, PathBuf};

use crate::polyglot::java::{
    i18n_class::I18nClass, i18n_enum::I18nEnum, i18n_mapper_initializer::I18nMapper,
    i18n_method::Method,
};

pub(crate) mod constants;
pub(crate) mod i18n_class;
pub(crate) mod i18n_enum;
pub(crate) mod i18n_mapper_initializer;
pub(crate) mod i18n_method;
pub(crate) mod pure_data_object;

pub(crate) struct JavaI18nGen {
    pub(crate) target_dir: PathBuf,
    class: I18nClass,
}

impl JavaI18nGen {
    pub fn new(class_name: &str) -> Self {
        Self {
            target_dir: "./".into(),
            class: I18nClass {
                class_name: Some(class_name.to_string()),
                methods: vec![Method::new_method_build_i18n_key(1)],
                i18n_enum: I18nEnum {
                    base_ident: 1,
                    ..Default::default()
                },
                i18n_mapper: I18nMapper {
                    base_ident: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        let codes = self.class.to_string();
        let target_dir = path.as_ref().to_path_buf();
        let class_name = self.class.get_class_name();
        let file_path = target_dir.join(format!("{class_name}.java"));
        std::fs::write(file_path, codes)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::polyglot::java::JavaI18nGen;

    #[test]
    fn test() {
        let mut i18n = JavaI18nGen::new("Test");
        i18n.class.add_i18n_entry("en", "test.title", "hello");
        i18n.class.add_i18n_entry("zh", "test.title", "你好");

        i18n.class
            .add_i18n_entry("en", "test.content", "this is content");
        i18n.class.add_i18n_entry("zh", "test.content", "这是内容");
        i18n.save_to("./codegen").unwrap();
    }
}
