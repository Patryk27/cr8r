use anyhow::Result;

use crate::backend::executor::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn do_invoke_cmd(&mut self, cmd: String) -> Result<()> {
        self.sandbox
            .exec(&cmd)
            .await
    }
}