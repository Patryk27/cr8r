use std::iter::FromIterator;

use toml::Value;
use toml::value::Table;

use crate::{CargoManifest, CargoManifestError, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CargoDependencyPatch<'a> {
    UseBranch(&'a str),
    UseTag(&'a str),
    UseVersion(&'a str),
    UsePath(&'a str),
}

impl CargoManifest {
    pub fn patch_dependency(&mut self, dep_name: &str, dep_patch: CargoDependencyPatch) -> Result<()> {
        let dep = if let Some(dep) = Self::load_dependency(&self.inner, dep_name)? {
            dep.clone()
        } else {
            return Ok(());
        };

        let dep_patch = match dep {
            // E.g.: foo = '0.1'
            dep @ Value::String(_) => {
                let dep = Table::from_iter(vec![
                    ("version".to_string(), dep),
                ].into_iter());

                Self::build_patch_for_non_git_dependency(dep, dep_name, dep_patch)?
            }

            // E.g.: foo = { version = '...', ... }
            Value::Table(dep) if dep.contains_key("version") => {
                Self::build_patch_for_non_git_dependency(dep, dep_name, dep_patch)?
            }

            // E.g.: foo = { path = '...', ... }
            Value::Table(dep) if dep.contains_key("path") => {
                Self::build_patch_for_non_git_dependency(dep, dep_name, dep_patch)?
            }

            // E.g.: foo = { git = '...', ... }
            Value::Table(dep) if dep.contains_key("git") => {
                Self::build_patch_for_git_dependency(dep, dep_patch)?
            }

            _ => {
                return Err(CargoManifestError::InvalidPropertyType {
                    path: format!("dependencies.{}", dep_name),
                    expected_type: "".to_string(),
                });
            }
        };

        Self::add_patch(&mut self.inner, dep_name, dep_patch)?;

        Ok(())
    }

    fn load_dependency<'a>(manifest: &'a Table, dep_name: &str) -> Result<Option<&'a Value>> {
        let deps = manifest
            .get("dependencies")
            .ok_or_else(|| CargoManifestError::MissingProperty {
                path: "dependencies".to_string(),
            })?
            .as_table()
            .ok_or_else(|| CargoManifestError::InvalidPropertyType {
                path: "dependencies".to_string(),
                expected_type: "table".to_string(),
            })?;

        Ok(deps.get(dep_name))
    }

    fn build_patch_for_non_git_dependency(
        mut dep: Table,
        dep_name: &str,
        dep_patch: CargoDependencyPatch,
    ) -> Result<ProcessedPatch> {
        let registry = dep
            .get("registry")
            .and_then(|reg| reg.as_str())
            .unwrap_or("crates-io")
            .to_string();

        dep.remove("path");
        dep.remove("version");

        match dep_patch {
            CargoDependencyPatch::UseBranch(_) => {
                return Err(CargoManifestError::IllegalDependencyPatch {
                    name: dep_name.to_string(),
                    reason: "Tried to apply a `branch = ...` patch on a non-Git dependency",
                });
            }

            CargoDependencyPatch::UseTag(_) => {
                return Err(CargoManifestError::IllegalDependencyPatch {
                    name: dep_name.to_string(),
                    reason: "Tried to apply a `tag = ...` patch on a non-Git dependency",
                });
            }

            CargoDependencyPatch::UseVersion(version) => {
                dep.insert("version".to_string(), Value::String(version.to_string()));
            }

            CargoDependencyPatch::UsePath(path) => {
                dep.insert("path".to_string(), Value::String(path.to_string()));
            }
        }

        Ok(ProcessedPatch {
            registry,
            content: Value::Table(dep),
        })
    }

    fn build_patch_for_git_dependency(mut dep: Table, dep_patch: CargoDependencyPatch) -> Result<ProcessedPatch> {
        let registry = dep
            .get("git")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        dep.remove("branch");
        dep.remove("tag");

        match dep_patch {
            CargoDependencyPatch::UseBranch(branch) => {
                dep.insert("branch".to_string(), Value::String(branch.to_string()));
            }

            CargoDependencyPatch::UseTag(tag) => {
                dep.insert("tag".to_string(), Value::String(tag.to_string()));
            }

            CargoDependencyPatch::UseVersion(version) => {
                dep.remove("git");
                dep.insert("version".to_string(), Value::String(version.to_string()));
            }

            CargoDependencyPatch::UsePath(path) => {
                dep.remove("git");
                dep.insert("path".to_string(), Value::String(path.to_string()));
            }
        }

        Ok(ProcessedPatch {
            registry,
            content: Value::Table(dep),
        })
    }

    fn add_patch(manifest: &mut Table, dep_name: &str, dep_patch: ProcessedPatch) -> Result<()> {
        let all_patches = manifest
            .entry("patch")
            .or_insert_with(|| Value::Table(Table::new()))
            .as_table_mut()
            .ok_or_else(|| CargoManifestError::InvalidPropertyType {
                path: "patch".to_string(),
                expected_type: "table".to_string(),
            })?;

        let registry_patches = all_patches
            .entry(dep_patch.registry.clone())
            .or_insert_with(|| Value::Table(Table::new()))
            .as_table_mut()
            .ok_or_else(|| CargoManifestError::InvalidPropertyType {
                path: format!("patch.{}", dep_patch.registry),
                expected_type: "table".to_string(),
            })?;

        registry_patches.insert(dep_name.to_string(), dep_patch.content);

        Ok(())
    }
}

struct ProcessedPatch {
    registry: String,
    content: Value,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_applying_patches_that_change_branch() {
        let expected = manifest("
            [dependencies]
            dep_branch  = { git = 'https://git.microsoft.com', branch = 'features/foo', features = ['foo', 'bar'] }
            dep_tag     = { git = 'https://git.microsoft.com', branch = 'features/foo', features = ['foo', 'bar'] }
            dep_version = { git = 'https://git.microsoft.com', branch = 'features/foo', features = ['foo', 'bar'] }

            [patch.'https://git.microsoft.com']
            dep_branch  = { git = 'https://git.microsoft.com', features = ['foo', 'bar'], branch = 'features/bar' }
            dep_tag     = { git = 'https://git.microsoft.com', features = ['foo', 'bar'], tag = 'v1.2.3.4' }
            dep_version = { features = ['foo', 'bar'], version = '1.2.3' }
        ");

        let actual = {
            let mut manifest = manifest("
                [dependencies]
                dep_branch  = { git = 'https://git.microsoft.com', branch = 'features/foo', features = ['foo', 'bar'] }
                dep_tag     = { git = 'https://git.microsoft.com', branch = 'features/foo', features = ['foo', 'bar'] }
                dep_version = { git = 'https://git.microsoft.com', branch = 'features/foo', features = ['foo', 'bar'] }
            ");

            manifest
                .patch_dependency("dep_branch", CargoDependencyPatch::UseBranch("features/bar"))
                .unwrap();

            manifest
                .patch_dependency("dep_tag", CargoDependencyPatch::UseTag("v1.2.3.4"))
                .unwrap();

            manifest
                .patch_dependency("dep_version", CargoDependencyPatch::UseVersion("1.2.3"))
                .unwrap();

            manifest
        };

        assert_manifest_eq(&expected, &actual);
    }

    #[test]
    fn test_applying_patches_that_change_tag() {
        let expected = manifest("
            [dependencies]
            dep_branch  = { git = 'https://git.microsoft.com', tag = 'features/foo', features = ['foo', 'bar'] }
            dep_tag     = { git = 'https://git.microsoft.com', tag = 'features/foo', features = ['foo', 'bar'] }
            dep_version = { git = 'https://git.microsoft.com', tag = 'features/foo', features = ['foo', 'bar'] }

            [patch.'https://git.microsoft.com']
            dep_branch  = { git = 'https://git.microsoft.com', features = ['foo', 'bar'], branch = 'features/bar' }
            dep_tag     = { git = 'https://git.microsoft.com', features = ['foo', 'bar'], tag = 'v1.2.3.4' }
            dep_version = { features = ['foo', 'bar'], version = '1.2.3' }
        ");

        let actual = {
            let mut manifest = manifest("
                [dependencies]
                dep_branch  = { git = 'https://git.microsoft.com', tag = 'features/foo', features = ['foo', 'bar'] }
                dep_tag     = { git = 'https://git.microsoft.com', tag = 'features/foo', features = ['foo', 'bar'] }
                dep_version = { git = 'https://git.microsoft.com', tag = 'features/foo', features = ['foo', 'bar'] }
            ");

            manifest
                .patch_dependency("dep_branch", CargoDependencyPatch::UseBranch("features/bar"))
                .unwrap();

            manifest
                .patch_dependency("dep_tag", CargoDependencyPatch::UseTag("v1.2.3.4"))
                .unwrap();

            manifest
                .patch_dependency("dep_version", CargoDependencyPatch::UseVersion("1.2.3"))
                .unwrap();

            manifest
        };

        assert_manifest_eq(&expected, &actual);
    }

    #[test]
    fn test_applying_patches_that_change_version() {
        let expected = manifest("
            [dependencies]
            dep_branch  = '0.1'
            dep_tag     = '0.1'
            dep_version = '0.1'

            [patch.crates-io]
            dep_version = { version = '1.2.3-beta' }
        ");

        let actual = {
            let mut manifest = manifest("
                [dependencies]
                dep_branch  = '0.1'
                dep_tag     = '0.1'
                dep_version = '0.1'
            ");

            assert_eq!(
                Err(CargoManifestError::IllegalDependencyPatch {
                    name: "dep_branch".to_string(),
                    reason: "Tried to apply a `branch = ...` patch on a non-Git dependency",
                }),
                manifest.patch_dependency("dep_branch", CargoDependencyPatch::UseBranch("features/bar"))
            );

            assert_eq!(
                Err(CargoManifestError::IllegalDependencyPatch {
                    name: "dep_tag".to_string(),
                    reason: "Tried to apply a `tag = ...` patch on a non-Git dependency",
                }),
                manifest.patch_dependency("dep_tag", CargoDependencyPatch::UseTag("v1.2.3.4"))
            );

            manifest
                .patch_dependency("dep_version", CargoDependencyPatch::UseVersion("1.2.3-beta"))
                .unwrap();

            manifest
        };

        assert_manifest_eq(&expected, &actual);
    }

    #[test]
    fn patching_non_existing_dependency_does_nothing() {
        let expected = manifest("
            [dependencies]
            foo = '0.1'
        ");

        let actual = {
            let mut manifest = manifest("
                [dependencies]
                foo = '0.1'
            ");

            manifest
                .patch_dependency("bar", CargoDependencyPatch::UseVersion("1.0"))
                .unwrap();

            manifest
        };

        assert_manifest_eq(&expected, &actual);
    }

    fn manifest(manifest: &str) -> CargoManifest {
        manifest
            .parse()
            .unwrap()
    }

    fn assert_manifest_eq(expected: &CargoManifest, actual: &CargoManifest) {
        assert_eq!(expected.print(), actual.print());
    }
}