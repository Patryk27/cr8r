use log::*;
use tokio::sync::{mpsc, oneshot};

use lib_protocol::core::{PAssignment, PExperimentId, PRunnerId, PRunnerName};
use lib_protocol::core::p_experiment_def::Op as PExperimentDefOp;

use crate::backend::{Experiment, Result, Runner};
use crate::backend::system::SystemActor;

pub type SystemTx = mpsc::UnboundedSender<SystemMsg>;
pub type SystemRx = mpsc::UnboundedReceiver<SystemMsg>;

#[derive(Debug)]
pub enum SystemMsg {
    FindExperiment {
        experiment: PExperimentId,
        tx: oneshot::Sender<Result<Experiment>>,
    },

    FindExperiments {
        tx: oneshot::Sender<Vec<Experiment>>,
    },

    FindRunners {
        tx: oneshot::Sender<Vec<Runner>>,
    },

    LaunchExperiment {
        experiment_def: PExperimentDefOp,
        tx: oneshot::Sender<Result<PExperimentId>>,
    },

    RegisterRunner {
        name: PRunnerName,
        tx: oneshot::Sender<Result<PRunnerId>>,
    },

    RequestAssignment {
        runner: PRunnerId,
        tx: oneshot::Sender<Result<Option<PAssignment>>>,
    },
}

mod find_experiment;
mod find_experiments;
mod find_runners;
mod launch_experiment;
mod register_runner;
mod request_assignment;

impl SystemMsg {
    pub async fn process(self, actor: &mut SystemActor) {
        debug!("Processing message: {:?}", self);

        match self {
            SystemMsg::FindExperiment { experiment, tx } => {
                let _ = tx.send(find_experiment::find_experiment(actor, experiment));
            }

            SystemMsg::FindExperiments { tx } => {
                let _ = tx.send(find_experiments::find_experiments(actor));
            }

            SystemMsg::FindRunners { tx } => {
                let _ = tx.send(find_runners::find_runners(actor));
            }

            SystemMsg::LaunchExperiment { experiment_def, tx } => {
                let _ = tx.send(launch_experiment::launch_experiment(actor, experiment_def));
            }

            SystemMsg::RegisterRunner { name, tx } => {
                let _ = tx.send(register_runner::register_runner(actor, name));
            }

            SystemMsg::RequestAssignment { runner, tx } => {
                let _ = tx.send(request_assignment::request_assignment(actor, runner).await);
            }
        };
    }
}