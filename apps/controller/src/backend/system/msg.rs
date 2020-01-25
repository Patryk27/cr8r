use anyhow::*;
use derivative::Derivative;
use log::*;
use tokio::sync::{mpsc, oneshot};

use lib_interop::domain::{DAssignment, DDefinition, DExperimentId, DRunnerId, DRunnerName};

use crate::backend::{Experiment, Runner};
use crate::backend::system::SystemActor;

pub type SystemTx = mpsc::UnboundedSender<SystemMsg>;
pub type SystemRx = mpsc::UnboundedReceiver<SystemMsg>;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum SystemMsg {
    CreateRunner {
        name: DRunnerName,

        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Result<DRunnerId>>,
    },

    FindRunners {
        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Vec<Runner>>,
    },

    // ---- //

    CreateExperiment {
        definition: DDefinition,

        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Result<DExperimentId>>,
    },

    FindExperiment {
        id: DExperimentId,

        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Result<Experiment>>,
    },

    FindExperiments {
        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Vec<Experiment>>,
    },

    // ---- //

    GetAssignment {
        runner_id: DRunnerId,

        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Result<Option<DAssignment>>>,
    },
}

mod create_runner;
mod find_runners;

mod create_experiment;
mod find_experiment;
mod find_experiments;

mod get_assignment;

impl SystemMsg {
    pub async fn handle(self, actor: &mut SystemActor) {
        debug!("Handling message: {:?}", self);

        match self {
            SystemMsg::CreateRunner { name, tx } => {
                let _ = tx.send(create_runner::create_runner(actor, name));
            }

            SystemMsg::FindRunners { tx } => {
                let _ = tx.send(find_runners::find_runners(actor));
            }

            // ---- //

            SystemMsg::CreateExperiment { definition, tx } => {
                let _ = tx.send(create_experiment::create_experiment(actor, definition));
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