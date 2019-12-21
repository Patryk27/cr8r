use crate::backend::executor::ExperimentExecutorActor;
use crate::backend::ExecutorStatus;

pub fn get_status(actor: &mut ExperimentExecutorActor) -> ExecutorStatus {
    actor.status
}