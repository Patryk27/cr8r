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
    pub fn apply_dependency_patch(&mut self, dep_name: &str, dep_patch: CargoDependencyPatch) -> Result<()> {
        use CargoDependencyPatch::*;

        let deps = self.inner
            .get("dependencies")
            .ok_or_else(|| CargoManifestError::MissingSection {
                name: "dependencies".to_string(),
            })?;

        let deps = deps
            .as_table()
            .ok_or_else(|| CargoManifestError::InvalidPropertyType {
                name: "dependencies".to_string(),
                expected_type: "table".to_string(),
            })?;

        let dep = if let Some(dep) = deps.get(dep_name) {
            dep.clone()
        } else {
            return Ok(());
        };

        let patch = match dep {
            // E.g.: foo = '0.1'
            Value::String(_) => {
                unimplemented!()
            }

            // E.g.: foo = { version = '0.1', ... }
            Value::Table(dep) if dep.contains_key("version") => {
                unimplemented!()
            }

            // E.g.: foo = { git = "...", ... }
            Value::Table(mut dep) if dep.contains_key("git") => {
                let registry = dep
                    .get("git")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();

                dep.remove("branch");
                dep.remove("tag");

                match dep_patch {
                    UseBranch(branch) => {
                        dep.insert("branch".to_string(), Value::String(branch.to_string()));
                    }

                    UseTag(tag) => {
                        dep.insert("tag".to_string(), Value::String(tag.to_string()));
                    }

                    UseVersion(version) => {
                        dep.remove("git");
                        dep.insert("version".to_string(), Value::String(version.to_string()));
                    }

                    UsePath(path) => {
                        dep.remove("git");
                        dep.insert("path".to_string(), Value::String(path.to_string()));
                    }
                }

                Patch {
                    registry,
                    content: Value::Table(dep),
                }
            }

            _ => {
                unimplemented!()
            }
        };

        let all_patches = self.inner
            .entry("patch")
            .or_insert_with(|| Value::Table(Table::new()))
            .as_table_mut()
            .ok_or_else(|| CargoManifestError::InvalidPropertyType {
                name: "patch".to_string(),
                expected_type: "table".to_string(),
            })?;

        let registry_patches = all_patches
            .entry(patch.registry.clone())
            .or_insert_with(|| Value::Table(Table::new()))
            .as_table_mut()
            .ok_or_else(|| CargoManifestError::InvalidPropertyType {
                name: format!("patch.{}", patch.registry),
                expected_type: "table".to_string(),
            })?;

        registry_patches.insert(dep_name.to_string(), patch.content);

        Ok(())
    }
}

struct Patch {
    registry: String,
    content: Value,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    mod when_applying_patch_from_branch {
        use super::*;

        #[test]
        fn into_branch_then_patch_succeeds() {
            let patch = CargoDependencyPatch::UseBranch("features/bar");

            let expected = manifest("
                [dependencies]
                dep = { git = 'https://git.microsoft.com', branch = 'features/foo', features = ['foo', 'bar'] }

                [patch.'https://git.microsoft.com']
                dep = { git = 'https://git.microsoft.com', features = ['foo', 'bar'], branch = 'features/bar' }
            ");

            test(patch, expected);
        }

        #[test]
        fn into_tag_then_patch_succeeds() {
            let patch = CargoDependencyPatch::UseTag("v1.2.3.4");

            let expected = manifest("
                [dependencies]
                dep = { git = 'https://git.microsoft.com', branch = 'features/foo', features = ['foo', 'bar'] }

                [patch.'https://git.microsoft.com']
                dep = { git = 'https://git.microsoft.com', features = ['foo', 'bar'], tag = 'v1.2.3.4' }
            ");

            test(patch, expected);
        }

        #[test]
        fn into_version_then_patch_succeeds() {
            let patch = CargoDependencyPatch::UseVersion("1.2.3.4");

            let expected = manifest("
                [dependencies]
                dep = { git = 'https://git.microsoft.com', branch = 'features/foo', features = ['foo', 'bar'] }

                [patch.'https://git.microsoft.com']
                dep = { features = ['foo', 'bar'], version = '1.2.3.4' }
            ");

            test(patch, expected);
        }

        fn test(patch: CargoDependencyPatch<'static>, expected: CargoManifest) {
            let mut actual = manifest("
                [dependencies]
                dep = { git = 'https://git.microsoft.com', branch = 'features/foo', features = ['foo', 'bar'] }
            ");

            actual
                .apply_dependency_patch("dep", patch)
                .unwrap();

            assert_manifest_eq(&expected, &actual);
        }
    }

    mod when_applying_patch_from_version {
        use super::*;

        #[test]
        fn into_branch_then_patch_fails() {
            let mut manifest = manifest();

            let result = manifest.apply_dependency_patch(
                "dep_concise",
                CargoDependencyPatch::UseBranch("foo"),
            );

            assert_eq!(result, Err(CargoManifestError::IllegalDependencyPatch {
                name: "dep_concise".to_string(),
                source: "Cannot apply `branch = ...` patch to a non-Git dependency",
            }));
        }

        #[test]
        fn into_tag_then_patch_fails() {
            let mut manifest = manifest();

            let result = manifest.apply_dependency_patch(
                "dep_concise",
                CargoDependencyPatch::UseTag("foo"),
            );

            assert_eq!(result, Err(CargoManifestError::IllegalDependencyPatch {
                name: "dep_concise".to_string(),
                source: "Cannot apply `tag = ...` patch to a non-Git dependency",
            }));
        }

        #[test]
        fn into_version_then_patch_succeeds() {
            let mut manifest = manifest();

            manifest
                .apply_dependency_patch("dep_concise", CargoDependencyPatch::UseVersion("1.0"))
                .unwrap();

            manifest
                .apply_dependency_patch("dep_expanded", CargoDependencyPatch::UseVersion("2.0-alpha"))
                .unwrap();

            let expected = super::manifest("
                [dependencies]
                dep_concise = '0.1'
                dep_expanded = { version = '0.1', features = ['foo', 'bar'] }
            ");

            assert_eq!(expected.print(), manifest.print());
        }

        #[test]
        fn into_path_then_patch_succeeds() {
            // @todo
        }

        fn manifest() -> CargoManifest {
            super::manifest("
                [dependencies]
                dep_concise = '0.1'
                dep_expanded = { version = '0.1', features = ['foo', 'bar'] }
            ")
        }
    }

    fn manifest(manifest: &str) -> CargoManifest {
        manifest
            .parse()
            .unwrap()
    }

    fn assert_manifest_eq(expected: &CargoManifest, actual: &CargoManifest) {
        assert_eq!(expected.print(), input.print());
    }
}