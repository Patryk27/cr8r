use crate::backend::{ExecutorActor, ExecutorMsg, ExecutorStatus};

impl ExecutorActor {
    pub(super) async fn process_messages(&mut self) {
        while let Ok(Some(msg)) = self.rx.try_next() {
            match msg {
                ExecutorMsg::Status { tx } => {
                    let _ = tx.send(self.status);
                }
            }
        }
    }

    fn status(&self) -> ExecutorStatus {
        self.status
    }
}