use thiserror::*;

use crate::cargo::CargoManifestMalformedError;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CargoManifestEditorError {
    #[error("Dependency `{name}` is not present in the `[dependencies]` section, so it cannot be patched")]
    CannotPatchUnknownDependency {
        name: String,
    },

    #[error("Manifest is malformed")]
    ManifestMalformed(
        #[from]
        #[source] CargoManifestMalformedError
    ),
}
