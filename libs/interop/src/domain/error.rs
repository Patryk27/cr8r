use std::result;

use thiserror::Error;

pub type DomainResult<T> = result::Result<T, DomainError>;

#[derive(Error, Debug)]
pub enum DomainError {
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
