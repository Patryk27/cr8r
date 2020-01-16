use anyhow::Result;

use lib_interop::domain::DEventType;

use super::super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn do_log_custom_msg(&mut self, msg: String) -> Result<()> {
        self.logger.add(DEventType::CustomMsg { msg });
        Ok(())
    }
}