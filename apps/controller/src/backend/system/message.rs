use futures_channel::{mpsc, oneshot};

use lib_protocol::core::{Assignment, ExperimentId, RunnerId, RunnerName};
use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

use crate::backend::{Experiment, Result, Runner};

pub type SystemTx = mpsc::UnboundedSender<SystemMsg>;
pub type SystemRx = mpsc::UnboundedReceiver<SystemMsg>;

#[derive(Debug)]
pub enum SystemMsg {
    // ---------------------------- //
    // Assignment-oriented messages //

    RequestAssignment {
        runner: RunnerId,
        tx: oneshot::Sender<Result<Option<Assignment>>>,
    },

    // ---------------------------- //
    // Experiment-oriented messages //

    FindExperiment {
        experiment: ExperimentId,
        tx: oneshot::Sender<Result<Experiment>>,
    },

    FindExperiments {
        tx: oneshot::Sender<Vec<Experiment>>,
    },

    LaunchExperiment {
        experiment: ExperimentDefinitionInner,
        tx: oneshot::Sender<Result<ExperimentId>>,
    },

    // ------------------------ //
    // Runner-oriented messages //

    FindRunners {
        tx: oneshot::Sender<Vec<Runner>>,
    },

    RegisterRunner {
        name: RunnerName,
        tx: oneshot::Sender<Result<RunnerId>>,
    },
}