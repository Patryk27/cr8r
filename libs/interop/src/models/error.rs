use std::result;

use thiserror::Error;

pub type ModelResult<T> = result::Result<T, ModelError>;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Field `{name}` is missing")]
    MissingField {
        name: &'static str,
    },

    #[error("Field `{name}` could not be read: {source:?}")]
    InvalidField {
        name: &'static str,
        source: anyhow::Error,
    },
}
