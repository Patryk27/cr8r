use thiserror::*;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CargoManifestMalformedError {
    #[error("Section `{name}` is missing")]
    MissingSection {
        name: String,
    },

    #[error("Property `{name}` was expected to be `{expected_type}`")]
    InvalidPropertyType {
        name: String,
        expected_type: String,
    },

    #[error("Manifest could not be serialized")]
    CannotBeSerialized(
        #[from]
        #[source] toml::ser::Error,
    ),

    #[error("Manifest could not be deserialized")]
    CannotBeUnserialized(
        #[from]
        #[source] toml::de::Error,
    ),
}