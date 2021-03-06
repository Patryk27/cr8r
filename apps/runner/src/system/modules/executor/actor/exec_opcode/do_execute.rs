use anyhow::*;

use super::super::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn do_execute(&mut self, cmd: String) -> Result<()> {
        self.sandbox.exec(&cmd).await
    }
}