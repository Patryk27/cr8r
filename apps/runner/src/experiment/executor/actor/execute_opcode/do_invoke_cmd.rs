use anyhow::Result;

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_invoke_cmd(&mut self, cmd: String) -> Result<()> {
        self.sandbox
            .exec(&cmd)
            .await
    }
}