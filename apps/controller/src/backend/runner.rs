use actix::Addr;
use actix::prelude::Request;

use lib_protocol_core::{ExperimentId, RunnerId, Scenario};

use crate::backend::System;

pub use self::actor::RunnerActor;

mod actor;
mod messages;

#[derive(Clone)]
pub struct Runner {
    addr: Addr<RunnerActor>,
}

impl Runner {
    pub fn create(system: System) -> RunnerActor {
        RunnerActor {
            id: RunnerId::new_v4(),
            system,
            authenticated: false,
        }
    }

    pub fn launch_experiment(&self, id: ExperimentId, scenarios: Vec<Scenario>) -> Request<RunnerActor, messages::LaunchExperiment> {
        self.addr.send(messages::LaunchExperiment { id, scenarios })
    }
}

impl From<Addr<RunnerActor>> for Runner {
    fn from(addr: Addr<RunnerActor>) -> Self {
        Self { addr }
    }
}