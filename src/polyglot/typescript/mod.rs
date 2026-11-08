use std::sync::Arc;

use crate::{IntoCode, args::Target, polyglot::common::i18n_code_generator::I18nCodeGenerator};

pub(crate) mod impl_typescript;

pub trait IntoTypeScript {
    fn into_typescript(&self) -> String;
}

#[derive(Debug)]
pub(crate) struct TypeScriptCodeGen {
    pub(crate) generator: Arc<I18nCodeGenerator>,
}

impl IntoCode for TypeScriptCodeGen {
    fn into_code(&self, _: Target) -> String {
        self.generator.as_ref().into_typescript()
    }
}
