use anyhow::Result;

use lib_interop::domain::DEventType;

use crate::backend::executor::ExecutorActor;

impl ExecutorActor {
    pub(super) async fn destroy_sandbox(&mut self) -> Result<()> {
        self.journalist.dispatch(DEventType::SystemMsg {
            msg: "Destroying sandbox".to_string(),
        });

        self.sandbox
            .destroy()
            .await
    }
}