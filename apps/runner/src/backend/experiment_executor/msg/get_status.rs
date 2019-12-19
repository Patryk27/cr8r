use crate::backend::experiment_executor::ExperimentExecutorActor;
use crate::backend::ExperimentExecutorStatus;

pub fn get_status(actor: &mut ExperimentExecutorActor) -> ExperimentExecutorStatus {
    actor.status
}