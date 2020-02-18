use anyhow::*;
use lib_interop::models::definition::DToolchainDef;

use super::super::ExecutorActor;

impl ExecutorActor {
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