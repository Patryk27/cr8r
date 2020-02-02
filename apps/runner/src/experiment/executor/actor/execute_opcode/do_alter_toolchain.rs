use anyhow::*;

use lib_interop::domain::definition::DToolchainDef;

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_alter_toolchain(
        &mut self,
        project: String,
        toolchain: DToolchainDef,
    ) -> Result<()> {
        self.sandbox.exec(&format!(
            "cd {} && rustup override set {}",
            project, toolchain.toolchain,
        )).await
    }
}