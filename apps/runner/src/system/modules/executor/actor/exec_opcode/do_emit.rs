use anyhow::*;

use lib_interop::models::DEventType;

use super::super::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn do_emit(&mut self, msg: String) -> Result<()> {
        self.logger.add(DEventType::SystemMsg { msg });
        Ok(())
    }
}