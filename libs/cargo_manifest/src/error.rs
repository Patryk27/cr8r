use core::result;

use thiserror::*;

pub type Result<T> = result::Result<T, CargoManifestError>;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CargoManifestError {
    #[error("Property `{path}` is missing")]
    MissingProperty {
        path: String,
    },

    #[error("Property `{path}` was expected to be of type `{expected_type}`")]
    InvalidPropertyType {
        path: String,
        expected_type: String,
    },

    #[error("Could not patch dependency `{name}`: {reason}")]
    IllegalDependencyPatch {
        name: String,
        reason: &'static str,
    },

    #[error("Could not serialize Cargo manifest")]
    CouldNotSerialize(
        #[from]
        #[source] toml::ser::Error,
    ),

    #[error("Could not deserialize Cargo manifest")]
    CouldNotDeserialize(
        #[from]
        #[source] toml::de::Error,
    ),
}
