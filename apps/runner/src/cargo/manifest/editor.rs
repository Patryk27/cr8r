use toml::value::Table;

pub use self::error::*;

mod error;
mod patch_dependency;

pub struct CargoManifestEditor {
    manifest: Table,
}

impl CargoManifestEditor {
    pub fn new(str: &str) -> Result<Self, CargoManifestEditorError> {
        let manifest = toml::from_str(str)
            .map_err(|err| CargoManifestEditorError::ManifestMalformed(err.into()))?;

        Ok(Self { manifest })
    }

    pub fn patch_dependency(
        &mut self,
        dep_name: &str,
        dep_version: &str,
    ) -> Result<(), CargoManifestEditorError> {
        patch_dependency::patch_dependency(&mut self.manifest, dep_name, dep_version)
    }

    pub fn finish(self) -> Result<String, CargoManifestEditorError> {
        toml::to_string_pretty(&self.manifest)
            .map_err(|err| CargoManifestEditorError::ManifestMalformed(err.into()))
    }

    fn into_inner(self) -> Table {
        self.manifest
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn making_no_changes_does_not_modify_the_manifest() {
        let editor = CargoManifestEditor::new(r#"
            [package]
            name = 'hello-world'
            version = '0.1.0'

            [dependencies]
            foo = '0.1'
        "#).unwrap();

        assert_eq!(
            "[package]\n\
             name = 'hello-world'\n\
             version = '0.1.0'\n\
             \n\
             [dependencies]\n\
             foo = '0.1'\n",
            editor.finish().unwrap(),
        );
    }
}