use std::path::PathBuf;

use anyhow::*;

use lib_interop::domain::job::opcode::DOverrideDependencyAction;

use crate::cargo::CargoManifestEditor;

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_override_dependency(
        &mut self,
        project: String,
        registry: String,
        name: String,
        action: DOverrideDependencyAction,
    ) -> Result<()> {
        let manifest_path = PathBuf::from(project)
            .join("Cargo.toml");

        let manifest = self.sandbox
            .fs_read(&manifest_path)
            .await
            .context("Could not read `Cargo.toml`")?;

        let manifest = {
            let mut editor = CargoManifestEditor::from_str(&manifest)?;

            match action {
                DOverrideDependencyAction::UseVersion { version } => {
                    editor.patch_dependency(&registry, &name, &version)?;
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
