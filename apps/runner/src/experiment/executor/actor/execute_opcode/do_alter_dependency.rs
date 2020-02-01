use std::path::PathBuf;
use std::str::FromStr;

use anyhow::*;

use lib_cargo_manifest::{CargoDependencyPatch, CargoManifest};
use lib_interop::domain::definition_inner::{DDependencyDef, DDependencyDefSource};

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_alter_dependency(
        &mut self,
        project: String,
        dependency: DDependencyDef,
    ) -> Result<()> {
        let manifest_path = PathBuf::from(project)
            .join("Cargo.toml");

        let manifest = self.sandbox
            .fs_read(&manifest_path)
            .await
            .context("Could not read `Cargo.toml`")?;

        let manifest = {
            let mut manifest = CargoManifest::from_str(&manifest)
                .context("Could not parse `Cargo.toml`")?;

            match dependency.source {
                DDependencyDefSource::Branch { branch } => {
                    manifest.apply_dependency_patch(&dependency.name, CargoDependencyPatch::UseBranch {
                        branch: &branch,
                    })
                }

                DDependencyDefSource::Tag { tag } => {
                    manifest.apply_dependency_patch(&dependency.name, CargoDependencyPatch::UseTag {
                        tag: &tag,
                    })
                }

                DDependencyDefSource::Version { version } => {
                    manifest.apply_dependency_patch(&dependency.name, CargoDependencyPatch::UseVersion {
                        version: &version,
                    })
                }

                _ => {
                    unimplemented!();
                }
            }.context("Could not update `Cargo.toml`")?;

            manifest
                .print()
                .context("Could not print `Cargo.toml`")?
        };

        self.sandbox
            .fs_write(manifest_path, manifest)
            .await
            .context("Could not write `Cargo.toml`")
    }
}
