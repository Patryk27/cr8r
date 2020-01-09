use anyhow::Result;

use lib_interop::domain::DEventType;

use crate::backend::executor::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn do_log_custom_msg(&mut self, msg: String) -> Result<()> {
        self.journalist.dispatch(DEventType::CustomMsg { msg });
        Ok(())
    }
}