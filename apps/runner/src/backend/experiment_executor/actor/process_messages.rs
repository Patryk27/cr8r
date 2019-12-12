use crate::backend::experiment_executor::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) async fn process_messages(&mut self) {
        while let Ok(Some(msg)) = self.rx.try_next() {
            msg.process(self);
        }
    }
}