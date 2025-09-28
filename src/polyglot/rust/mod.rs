use std::sync::Arc;

use crate::{
    IntoCode,
    args::Target,
    polyglot::common::{Naming, i18n_code_generator::I18nCodeGenerator},
};

pub(crate) mod impl_rust;

#[derive(Debug, Clone)]
pub struct RustSettings {
    pub formatx_enable: bool,
}

pub trait IntoRust {
    fn into_rust(&self, settings: &RustSettings) -> String;
}

pub(crate) trait IntoRustConstruct {
    fn construct_rust(&self, base_ident: u8, args: Vec<Naming>) -> String;
}

#[derive(Debug)]
pub(crate) struct RustCodeGen {
    pub(crate) generator: Arc<I18nCodeGenerator>,
}

impl IntoCode for RustCodeGen {
    fn into_code(&self, target: Target) -> String {
        let formatx_enable = match target {
            Target::Rust { formatx_enable } => formatx_enable,
            _ => unreachable!(),
        };
        self.generator.into_rust(&RustSettings { formatx_enable })
    }
}
