use actix::{Handler, Message, MessageResult};

use lib_protocol as proto;

use crate::modules::System;

#[derive(Message)]
#[rtype(result = "proto::ControllerStatus")]
pub struct GetControllerStatus;

impl Handler<GetControllerStatus> for System {
    type Result = MessageResult<GetControllerStatus>;

    fn handle(&mut self, msg: GetControllerStatus, ctx: &mut Self::Context) -> Self::Result {
        let experiments = self.experiments.iter().map(|(id, experiment)| {
            proto::Experiment {
                id: id.clone(),
                status: experiment.status.clone(),
                definition: experiment.definition.clone(),
            }
        }).collect();

        let runners = self.runners.iter().map(|(id, runner)| {
            proto::Runner {
                id: *id,
                name: runner.name.clone(),
                status: runner.status.clone(),
            }
        }).collect();

        MessageResult(proto::ControllerStatus { experiments, runners })
    }
}