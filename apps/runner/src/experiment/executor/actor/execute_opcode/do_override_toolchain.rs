use anyhow::*;

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_override_toolchain(
        &mut self,
        project: String,
        tc_version: String,
    ) -> Result<()> {
        self.sandbox.exec(&format!(
            "cd {} && rustup override set {}",
            project, tc_version,
        )).await
    }
}