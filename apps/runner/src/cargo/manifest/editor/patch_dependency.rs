use toml::Value;
use toml::value::Table;

use crate::cargo::{CargoManifestEditorError, CargoManifestMalformedError};

pub fn patch_dependency(
    manifest: &mut Table,
    dep_name: &str,
    dep_patch: &str,
) -> Result<(), CargoManifestEditorError> {
    let deps = manifest
        .get("dependencies")
        .ok_or_else(|| CargoManifestMalformedError::MissingSection {
            name: "dependencies".to_string(),
        })?
        .as_table()
        .ok_or_else(|| CargoManifestMalformedError::InvalidPropertyType {
            name: "dependencies".to_string(),
            expected_type: "table".to_string(),
        })?;

    let dep = deps
        .get(dep_name)
        .ok_or_else(|| CargoManifestEditorError::CannotPatchUnknownDependency {
            name: dep_name.to_string()
        })?;

    let patch = match dep {
        // E.g.: foo = '0.1'
        Value::String(_) => {
            Patch {
                registry: "crates-io".to_string(),
                content: Value::String(dep_patch.to_string()),
            }
        }

        // E.g.: foo = { version = '0.1' }
        Value::Table(dep) if dep.get("version").is_some() => {
            let mut dep = dep.clone();

            dep["version"] = Value::String(dep_patch.to_string());

            Patch {
                registry: "crates-io".to_string(),
                content: Value::Table(dep),
            }
        }

        // Unknown dependency source
        _ => {
            unimplemented!()
        }
    };

    manifest
        .entry("patch")
        .or_insert_with(|| Value::Table(Table::new()))
        .as_table_mut()
        .ok_or_else(|| CargoManifestMalformedError::InvalidPropertyType {
            name: "patch".to_string(),
            expected_type: "table".to_string(),
        })?
        //
        .entry(patch.registry.clone())
        .or_insert_with(|| Value::Table(Table::new()))
        .as_table_mut()
        .ok_or_else(|| CargoManifestMalformedError::InvalidPropertyType {
            name: format!("patch.{}", patch.registry),
            expected_type: "table".to_string(),
        })?
        //
        .insert(dep_name.to_string(), patch.content);

    Ok(())
}

struct Patch {
    registry: String,
    content: Value,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use super::super::CargoManifestEditor;

    #[test]
    fn patching_crates_io_dependency_adds_patch_to_the_crates_io_section() {
        let actual_manifest = {
            let mut editor = editor("
                [package]
                name = 'hello-world'
                version = '0.1.0'

                [dependencies]
                pkg_concise = '0.1'
                pkg_expanded = { version = '0.1', features = ['foo', 'bar'] }
            ");

            editor.patch_dependency("pkg_concise", "1.0").unwrap();
            editor.patch_dependency("pkg_expanded", "2.0-alpha").unwrap();
            editor.into_inner()
        };

        let expected_manifest = manifest("
            [package]
            name = 'hello-world'
            version = '0.1.0'

            [dependencies]
            pkg_concise = '0.1'
            pkg_expanded = { version = '0.1', features = ['foo', 'bar'] }

            [patch.crates-io]
            pkg_concise = '1.0'
            pkg_expanded = { version = '2.0-alpha', features = ['foo', 'bar'] }
        ");

        assert(expected_manifest, actual_manifest);
    }

    fn editor(manifest: &str) -> CargoManifestEditor {
        CargoManifestEditor::new(manifest)
            .unwrap()
    }

    fn manifest(manifest: &str) -> Table {
        toml::from_str(manifest)
            .unwrap()
    }

    fn assert(expected_manifest: Table, actual_manifest: Table) {
        assert_eq!(
            toml::to_string_pretty(&expected_manifest),
            toml::to_string_pretty(&actual_manifest),
        );
    }
}