use std::path::PathBuf;

use anyhow::*;

use lib_interop::domain::definition_inner::{DDependencyDef, DDependencyDefSource};

use crate::cargo::CargoManifestEditor;

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
            let mut editor = CargoManifestEditor::new(&manifest)?;

            match dependency.source {
                DDependencyDefSource::Version { version } => {
                    editor.patch_dependency(&dependency.name, &version)?;
                }

                _ => {
                    unimplemented!();
                }
            };

            editor.finish()?
        };

        self.sandbox
            .fs_write(manifest_path, manifest)
            .await
            .context("Could not write `Cargo.toml`")
    }
}
