use std::collections::{HashMap, VecDeque};

use actix::{Actor, AsyncContext, Context, WrapFuture};
use futures::Future;
use log::*;

use lib_protocol_core::{Experiment, ExperimentId, ExperimentStatus, RunnerId, RunnerStatus};

use crate::backend::Compiler;

pub use self::runner_entry::*;

mod runner_entry;

pub struct SystemActor {
    pub runner_secret: String,
    pub compiler: Compiler,

    pub runners: HashMap<RunnerId, RunnerEntry>,
    pub idle_runners: VecDeque<RunnerId>,

    pub experiments: HashMap<ExperimentId, Experiment>,
    pub awaiting_experiments: VecDeque<ExperimentId>,
}

impl SystemActor {
    pub fn process_waiting_experiments(&mut self, ctx: &mut Context<Self>) {
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

            let fut = runner.runner.launch_experiment(
                experiment.id,
                experiment.scenarios.clone(),
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