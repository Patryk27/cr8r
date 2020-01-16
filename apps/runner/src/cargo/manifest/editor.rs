use toml::Value;
use toml::value::Table;

use crate::cargo::{CargoManifestError, CargoManifestMalformedError};

pub struct CargoManifestEditor {
    manifest: Table,
}

impl CargoManifestEditor {
    pub fn from_str(str: &str) -> Result<Self, CargoManifestError> {
        Ok(Self {
            manifest: toml::from_str(str)?,
        })
    }

    pub fn patch_dependency(&mut self, registry: &str, name: &str, version: &str) -> Result<(), CargoManifestError> {
        let patch = self.manifest
            .entry("patch")
            .or_insert_with(|| Value::Table(Table::new()))
            .as_table_mut()
            .ok_or_else(|| CargoManifestMalformedError::InvalidSectionType {
                name: "patch".to_string(),
                expected: "table".to_string(),
            })?;

        let registry_patch = patch
            .entry(registry)
            .or_insert_with(|| Value::Table(Table::new()))
            .as_table_mut()
            .ok_or_else(|| CargoManifestMalformedError::InvalidSectionType {
                name: format!("patch.{}", registry),
                expected: "table".to_string(),
            })?;

        let dependency_patch = registry_patch
            .entry(name)
            .or_insert_with(|| Value::String("".to_string()));

        match dependency_patch {
            Value::String(dep_version) => {
                *dep_version = version.to_string();
            }

            _ => {
                Err(CargoManifestMalformedError::InvalidSectionType {
                    name: format!("patch.{}.{}", registry, name),
                    expected: "table".to_string(),
                })?;
            }
        }

        Ok(())
    }

    pub fn finish(self) -> Result<String, CargoManifestError> {
        toml::to_string_pretty(&self.manifest)
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn no_changes() {
        let editor = CargoManifestEditor::from_str(r#"
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

    mod patch_dependency {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn simple() {
            let mut editor = CargoManifestEditor::from_str(r#"
                [package]
                name = 'hello-world'
                version = '0.1.0'

                [dependencies]
                foo = '0.1'
                bar = '0.2'
            "#).unwrap();

            editor
                .patch_dependency("crates-io", "foo", "1.0-alpha")
                .unwrap();

            editor
                .patch_dependency("https://git.hello.world", "bar", "2.0-beta")
                .unwrap();

            assert_eq!(
                "[package]\n\
                 name = 'hello-world'\n\
                 version = '0.1.0'\n\
                 \n\
                 [dependencies]\n\
                 foo = '0.1'\n\
                 bar = '0.2'\n\
                 [patch.crates-io]\n\
                 foo = '1.0-alpha'\n\
                 \n\
                 [patch.\"https://git.hello.world\"]\n\
                 bar = '2.0-beta'\n",
                editor.finish().unwrap(),
            );
        }
    }
}