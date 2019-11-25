use std::collections::{HashMap, VecDeque};

use actix::{Actor, Addr};
use actix::prelude::Request;

use lib_protocol_core::{ExperimentDefinition, RunnerId, RunnerName};

use crate::backend::{Compiler, Runner};

pub use self::actor::SystemActor;

mod actor;
mod messages;

#[derive(Clone)]
pub struct System {
    addr: Addr<SystemActor>,
}

impl System {
    pub fn spawn(runner_secret: String, compiler: Compiler) -> Self {
        let addr = SystemActor {
            runner_secret,
            compiler,

            runners: HashMap::new(),
            idle_runners: VecDeque::new(),

            experiments: HashMap::new(),
            awaiting_experiments: VecDeque::new(),
        }.start();

        Self { addr }
    }

    pub fn status(&self) -> Request<SystemActor, messages::GetControllerStatus> {
        self.addr.send(messages::GetControllerStatus)
    }

    pub fn launch_experiment(&self, definition: ExperimentDefinition) -> Request<SystemActor, messages::CreateExperiment> {
        self.addr.send(messages::CreateExperiment { definition })
    }

    pub fn authenticate_runner(&self, runner: Runner, id: RunnerId, name: RunnerName, secret: String) -> Request<SystemActor, messages::AuthenticateRunner> {
        self.addr.send(messages::AuthenticateRunner { runner, id, name, secret })
    }

    pub fn deauthenticate_runner(&self, id: RunnerId) -> Request<SystemActor, messages::DeauthenticateRunner> {
        self.addr.send(messages::DeauthenticateRunner { id })
    }

    pub fn unpark_runner(&self, id: RunnerId) -> Request<SystemActor, messages::UnparkRunner> {
        self.addr.send(messages::UnparkRunner { id })
    }
}