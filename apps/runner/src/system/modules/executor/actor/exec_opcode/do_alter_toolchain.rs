use anyhow::*;

use lib_interop::models::definition::DToolchainDef;

use super::super::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn do_alter_toolchain(
        &mut self,
        altered_project: String,
        altered_toolchain: DToolchainDef,
    ) -> Result<()> {
        self.sandbox.exec(&format!(
            "cd {} && rustup override set {}",
            altered_project,
            altered_toolchain.toolchain,
        )).await
    }
}