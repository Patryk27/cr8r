use log::*;
use tokio::sync::{mpsc, oneshot};

use lib_interop::contract::{CAssignment, CExperimentDef, CExperimentId, CRunnerId, CRunnerName};

use crate::backend::{Experiment, Result, Runner};
use crate::backend::system::SystemActor;

pub type SystemTx = mpsc::UnboundedSender<SystemMsg>;
pub type SystemRx = mpsc::UnboundedReceiver<SystemMsg>;

#[derive(Debug)]
pub enum SystemMsg {
    CreateRunner {
        name: CRunnerName,
        tx: oneshot::Sender<Result<CRunnerId>>,
    },

    FindRunners {
        tx: oneshot::Sender<Vec<Runner>>,
    },

    // ---- //

    CreateExperiment {
        def: CExperimentDef,
        tx: oneshot::Sender<Result<CExperimentId>>,
    },

    FindExperiment {
        id: CExperimentId,
        tx: oneshot::Sender<Result<Experiment>>,
    },

    FindExperiments {
        tx: oneshot::Sender<Vec<Experiment>>,
    },

    // ---- //

    GetAssignment {
        runner_id: CRunnerId,
        tx: oneshot::Sender<Result<Option<CAssignment>>>,
    },
}

mod create_runner;
mod find_runners;

mod create_experiment;
mod find_experiment;
mod find_experiments;

mod get_assignment;

impl SystemMsg {
    pub async fn process(self, actor: &mut SystemActor) {
        debug!("Processing message: {:?}", self);

        match self {
            SystemMsg::CreateRunner { name, tx } => {
                let _ = tx.send(create_runner::create_runner(actor, name));
            }

            SystemMsg::FindRunners { tx } => {
                let _ = tx.send(find_runners::find_runners(actor));
            }

            // ---- //

            SystemMsg::CreateExperiment { def, tx } => {
                let _ = tx.send(create_experiment::create_experiment(actor, def));
            }

            SystemMsg::FindExperiment { id, tx } => {
                let _ = tx.send(find_experiment::find_experiment(actor, id));
            }

            SystemMsg::FindExperiments { tx } => {
                let _ = tx.send(find_experiments::find_experiments(actor));
            }

            // ---- //

            SystemMsg::GetAssignment { runner_id, tx } => {
                let _ = tx.send(get_assignment::get_assignment(actor, runner_id).await);
            }
        };
    }
}