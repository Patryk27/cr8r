use tokio::stream::StreamExt;

use crate::backend::experiment_executor::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    /// Processes all pending actor messages and then yields control back to the caller.
    pub(super) fn process_messages_and_yield(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            msg.process(self);
        }
    }

    /// Processes all pending actor messages and then waits for more, blocking in an async-fashion.
    pub(super) async fn process_messages_and_wait(&mut self) {
        while let Some(msg) = self.rx.next().await {
            msg.process(self);
        }
    }
}