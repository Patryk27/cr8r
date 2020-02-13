use std::path::PathBuf;
use std::str::FromStr;

use anyhow::*;

use lib_cargo_manifest::{CargoDependencyPatch, CargoManifest};
use lib_interop::domain::definition::{DDependencyDef, DDependencySourceDef};

use super::super::ExecutorActor;

impl ExecutorActor {
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
                DDependencySourceDef::Branch { branch } => {
                    manifest.patch_dependency(
                        &dependency.name,
                        CargoDependencyPatch::UseBranch(&branch),
                    )
                }

                DDependencySourceDef::Tag { tag } => {
                    manifest.patch_dependency(
                        &dependency.name,
                        CargoDependencyPatch::UseTag(&tag),
                    )
                }

                DDependencySourceDef::Version { version } => {
                    manifest.patch_dependency(
                        &dependency.name,
                        CargoDependencyPatch::UseVersion(&version),
                    )
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
