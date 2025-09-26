use std::sync::Arc;

use crate::{IntoCode, polyglot::common::i18n_code_generator::I18nCodeGenerator};

pub(crate) mod impl_java;

pub trait IntoJava {
    fn into_java(&self) -> String;
}

pub(crate) trait IntoJavaConstruct {
    fn construct_java(&self, args: Vec<&str>) -> String;
}

pub(crate) trait IntoJavaConstructor {
    fn into_constructor(&self) -> String;
}

#[derive(Debug)]
pub(crate) struct JavaCodeGen {
    pub(crate) generator: Arc<I18nCodeGenerator>,
}

impl IntoCode for JavaCodeGen {
    fn into_code(&self) -> String {
        self.generator.as_ref().into_java()
    }
}
