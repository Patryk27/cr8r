use anyhow::*;

use lib_interop::domain::DEventType;

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_log_system_msg(&mut self, msg: String) -> Result<()> {
        self.logger.add(DEventType::SystemMsg { msg });
        Ok(())
    }
}