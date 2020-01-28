use anyhow::*;

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_execute(&mut self, cmd: String) -> Result<()> {
        self.sandbox
            .exec(&cmd)
            .await
    }
}