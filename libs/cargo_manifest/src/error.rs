use core::result;

use thiserror::*;

pub type Result<T> = result::Result<T, CargoManifestError>;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CargoManifestError {
    #[error("Section `{name}` is missing")]
    MissingSection {
        name: String,
    },

    #[error("Property `{name}` was expected to be `{expected_type}`")]
    InvalidPropertyType {
        name: String,
        expected_type: String,
    },

    #[error("Could not patch dependency `{name}`")]
    IllegalDependencyPatch {
        name: String,
        #[source] source: &'static str,
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