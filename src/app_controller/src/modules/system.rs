use std::collections::{HashMap, VecDeque};

use actix::{Actor, Addr, AsyncContext, Context, WrapFuture};
use actix::prelude::Request;
use futures::Future;
use log::*;

use lib_protocol::{Experiment, ExperimentDefinition, ExperimentId, ExperimentStatus, RunnerId, RunnerName, RunnerStatus};

use crate::modules::{Compiler, RunnerActor, RunnerEntry};

mod messages;

pub struct SystemActor {
    secret: String,
    compiler: Compiler,

    runners: HashMap<RunnerId, RunnerEntry>,
    idle_runners: VecDeque<RunnerId>,

    experiments: HashMap<ExperimentId, Experiment>,
    awaiting_experiments: VecDeque<ExperimentId>,
}

impl SystemActor {
    pub fn new(secret: String, compiler: Compiler) -> Self {
        Self {
            secret,
            compiler,

            runners: HashMap::new(),
            idle_runners: VecDeque::new(),

            experiments: HashMap::new(),
            awaiting_experiments: VecDeque::new(),
        }
    }

    pub fn get_status(addr: &Addr<Self>) -> Request<Self, messages::GetControllerStatus> {
        addr.send(messages::GetControllerStatus)
    }

    pub fn create_experiment(
        addr: &Addr<Self>,
        definition: ExperimentDefinition,
    ) -> Request<Self, messages::CreateExperiment> {
        addr.send(messages::CreateExperiment { definition })
    }

    pub fn authenticate_runner(
        addr: &Addr<Self>,
        actor: Addr<RunnerActor>,
        id: RunnerId,
        name: RunnerName,
        secret: String,
    ) -> Request<Self, messages::AuthenticateRunner> {
        addr.send(messages::AuthenticateRunner { actor, id, name, secret })
    }

    pub fn deauthenticate_runner(
        addr: &Addr<Self>,
        id: RunnerId,
    ) -> Request<Self, messages::DeauthenticateRunner> {
        addr.send(messages::DeauthenticateRunner { id })
    }

    pub fn unpark_runner(
        addr: &Addr<Self>,
        id: RunnerId,
    ) -> Request<Self, messages::UnparkRunner> {
        addr.send(messages::UnparkRunner { id })
    }

    fn process_waiting_experiments(&mut self, ctx: &mut Context<Self>) {
        while let (Some(_), Some(_)) = (self.idle_runners.front(), self.awaiting_experiments.front()) {
            let runner_id = self.idle_runners
                .pop_front()
                .unwrap();

            let experiment_id = self.awaiting_experiments
                .pop_front()
                .unwrap();

            let runner = self.runners
                .get_mut(&runner_id)
                .unwrap();

            let experiment = self.experiments
                .get_mut(&experiment_id)
                .unwrap();

            runner.status = RunnerStatus::Working {
                experiment_id,
            };

            experiment.status = ExperimentStatus::Running {
                runner_id,
            };

            let fut = RunnerActor::launch_experiment(
                &runner.actor,
                experiment.id,
                experiment.execution_plans.clone(),
            );

            ctx.wait(
                fut.map_err(|_| ()).into_actor(self)
            );
        }
    }
}

impl Actor for SystemActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        debug!("Actor started.");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        debug!("Actor stopped.");
    }
}