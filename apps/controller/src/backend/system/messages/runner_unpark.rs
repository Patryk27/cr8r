use actix::{Handler, Message};
use log::*;

use lib_protocol_core::{RunnerId, RunnerStatus};

use crate::backend::SystemActor;

#[derive(Message)]
pub struct UnparkRunner {
    pub id: RunnerId,
}

impl Handler<UnparkRunner> for SystemActor {
    type Result = ();

    fn handle(&mut self, msg: UnparkRunner, ctx: &mut Self::Context) -> Self::Result {
        debug!("Runner `{}` has been un-parked.", msg.id);

        self.runners
            .get_mut(&msg.id)
            .unwrap()
            .status = RunnerStatus::Idle;

        self.idle_runners.push_back(msg.id);
        self.process_waiting_experiments(ctx);
    }
}