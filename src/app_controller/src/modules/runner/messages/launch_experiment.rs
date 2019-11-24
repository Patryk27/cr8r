use actix::{Handler, Message};
use log::*;

use lib_protocol::{ExecutionPlan, ExperimentId};
use lib_protocol::targets::runner::RunnerMessage;

use crate::modules::RunnerActor;

#[derive(Message)]
pub struct LaunchExperiment {
    pub id: ExperimentId,
    pub plans: Vec<ExecutionPlan>,
}

impl Handler<LaunchExperiment> for RunnerActor {
    type Result = ();

    fn handle(&mut self, msg: LaunchExperiment, ctx: &mut Self::Context) -> Self::Result {
        // @todo we should ensure the runner acknowledged the command

        info!("Runner `{}` has been assigned experiment `{}`.", self.id, msg.id);

        ctx.text(RunnerMessage::LaunchExperiment {
            id: msg.id,
            plans: msg.plans,
        });
    }
}