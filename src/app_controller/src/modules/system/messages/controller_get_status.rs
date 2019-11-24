use actix::{Handler, Message, MessageResult};

use lib_protocol::ControllerStatus;

use crate::modules::{RunnerEntry, SystemActor};

#[derive(Message)]
#[rtype(result = "ControllerStatus")]
pub struct GetControllerStatus;

impl Handler<GetControllerStatus> for SystemActor {
    type Result = MessageResult<GetControllerStatus>;

    fn handle(&mut self, _: GetControllerStatus, _: &mut Self::Context) -> Self::Result {
        MessageResult(ControllerStatus {
            experiments: self.experiments
                .values()
                .cloned()
                .collect(),

            runners: self.runners
                .values()
                .map(RunnerEntry::as_model)
                .collect(),
        })
    }
}