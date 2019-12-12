use crate::backend::experiment_executor::ExperimentExecutorActor;
use crate::backend::ExperimentExecutorStatus;

pub fn process(actor: &mut ExperimentExecutorActor) -> ExperimentExecutorStatus {
    actor.status
}