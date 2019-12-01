use futures_channel::{mpsc, oneshot};

use lib_protocol::core::{Assignment, ExperimentId, RunnerId, RunnerName, RunnerSecret};
use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

use crate::backend::{Experiment, Result};

pub type SystemCommandTx = mpsc::UnboundedSender<SystemCommand>;
pub type SystemCommandRx = mpsc::UnboundedReceiver<SystemCommand>;

#[derive(Debug)]
pub enum SystemCommand {
    // --------------------------- //
    // Assignment-related commands //

    RequestAssignment {
        runner: RunnerId,
        tx: oneshot::Sender<Result<Option<Assignment>>>,
    },

    // --------------------------- //
    // Experiment-related commands //

    FindExperiment {
        experiment: ExperimentId,
        tx: oneshot::Sender<Result<Experiment>>,
    },

    LaunchExperiment {
        experiment: ExperimentDefinitionInner,
        tx: oneshot::Sender<Result<ExperimentId>>,
    },

    // ----------------------- //
    // Runner-related commands //

    RegisterRunner {
        name: RunnerName,
        secret: RunnerSecret,
        tx: oneshot::Sender<Result<RunnerId>>,
    },
}