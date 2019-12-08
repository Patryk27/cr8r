use crate::backend::{ExecutorStatus, ExperimentExecutorActor, ExperimentExecutorMsg};

impl ExperimentExecutorActor {
    pub(super) async fn process_messages(&mut self) {
        while let Ok(Some(msg)) = self.rx.try_next() {
            match msg {
                ExperimentExecutorMsg::Status { tx } => {
                    let _ = tx.send(self.status);
                }
            }
        }
    }

    fn status(&self) -> ExecutorStatus {
        self.status
    }
}