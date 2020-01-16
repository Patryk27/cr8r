use thiserror::*;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CargoManifestError {
    #[error("Manifest is malformed")]
    Malformed(
        #[from]
        #[source] CargoManifestMalformedError
    ),

    #[error("Manifest could not be serialized")]
    SerializerError(
        #[from]
        #[source] toml::ser::Error
    ),

    #[error("Manifest could not be deserialized")]
    DeserializerError(
        #[from]
        #[source] toml::de::Error
    ),
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CargoManifestMalformedError {
    #[error("Section `{name}` was expected to be `{expected}`")]
    InvalidSectionType {
        name: String,
        expected: String,
    }
}