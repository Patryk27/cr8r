use crate::backend::executor::ExecutorActor;
use crate::backend::ExecutorStatus;

pub fn get_status(actor: &mut ExecutorActor) -> ExecutorStatus {
    actor.status
}