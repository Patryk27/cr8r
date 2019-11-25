use actix::{Handler, Message};
use log::*;

use lib_protocol_core::{ExperimentId, Scenario};
use lib_runner_protocol::RunnerMessage;

use crate::backend::RunnerActor;

#[derive(Message)]
pub struct LaunchExperiment {
    pub id: ExperimentId,
    pub scenarios: Vec<Scenario>,
}

impl Handler<LaunchExperiment> for RunnerActor {
    type Result = ();

    fn handle(&mut self, msg: LaunchExperiment, ctx: &mut Self::Context) -> Self::Result {
        // @todo we should ensure the runner acknowledged the command

        info!("Runner `{}` has been assigned experiment `{}`.", self.id, msg.id);

        ctx.text(RunnerMessage::LaunchExperiment {
            id: msg.id,
            scenarios: msg.scenarios,
        });
    }
}