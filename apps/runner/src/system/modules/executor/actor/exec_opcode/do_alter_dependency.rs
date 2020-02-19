use std::path::PathBuf;
use std::str::FromStr;

use anyhow::*;

use lib_cargo_manifest::{CargoDependencyPatch, CargoManifest};
use lib_interop::models::definition::{DDependencyDef, DDependencySourceDef};

use super::super::{ExecutorActor, ExecutorContext};

impl ExecutorActor {
    pub(super) async fn do_alter_dependency(
        &mut self,
        context: &ExecutorContext,
        altered_project: String,
        altered_dependency: DDependencyDef,
    ) -> Result<()> {
        let manifest_path = PathBuf::from(altered_project)
            .join("Cargo.toml");

        let manifest = self.sandbox
            .fs_read(&manifest_path).await
            .context("Could not read `Cargo.toml`")?;

        let mut manifest = CargoManifest::from_str(&manifest)
            .context("Could not parse `Cargo.toml`")?;

        match altered_dependency.source {
            DDependencySourceDef::Branch { branch } => {
                manifest.patch_dependency(
                    &altered_dependency.name,
                    CargoDependencyPatch::UseBranch(&branch),
                )
            }

            DDependencySourceDef::Tag { tag } => {
                manifest.patch_dependency(
                    &altered_dependency.name,
                    CargoDependencyPatch::UseTag(&tag),
                )
            }

            DDependencySourceDef::Version { version } => {
                manifest.patch_dependency(
                    &altered_dependency.name,
                    CargoDependencyPatch::UseVersion(&version),
                )
            }

            DDependencySourceDef::Path { attachment_id } => {
                let attachment = context.attachments
                    .get(&attachment_id)
                    .unwrap();

                let attachment_path = attachment
                    .path()
                    .display()
                    .to_string();

                // @todo this won't work with LXD!

                manifest.patch_dependency(
                    &altered_dependency.name,
                    CargoDependencyPatch::UsePath(&attachment_path),
                )
            }
        }.context("Could not update `Cargo.toml`")?;

        let manifest = manifest
            .print()
            .context("Could not print `Cargo.toml`")?;

        self.sandbox
            .fs_write(manifest_path, manifest).await
            .context("Could not write `Cargo.toml`")
    }
}
