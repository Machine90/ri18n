use std::sync::Arc;

use crate::{IntoCode, args::Target, polyglot::common::i18n_code_generator::I18nCodeGenerator};

pub(crate) mod impl_python;

pub trait IntoPython {
    fn into_python(&self) -> String;
}

#[derive(Debug)]
pub(crate) struct PythonCodeGen {
    pub(crate) generator: Arc<I18nCodeGenerator>,
}

impl IntoCode for PythonCodeGen {
    fn into_code(&self, _: Target) -> String {
        self.generator.as_ref().into_python()
    }
}
