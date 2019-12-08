use futures_channel::mpsc;

use lib_actor::ask;
use lib_protocol::core::Assignment;
use lib_sandbox::Sandbox;

use crate::backend::ExperimentReporter;
use crate::core::ExperimentClient;

pub use self::{
    actor::*,
    error::*,
    msg::*,
    status::*,
};

mod actor;
mod error;
mod msg;
mod status;

pub struct ExperimentExecutor {
    tx: ExperimentExecutorTx,
}

impl ExperimentExecutor {
    pub fn spawn(sandbox: Sandbox, assignment: Assignment, client: ExperimentClient) -> Self {
        let (tx, rx) = mpsc::unbounded();

        let reporter = ExperimentReporter::spawn(client);

        tokio::spawn(ExperimentExecutorActor::new(
            rx,
            sandbox,
            assignment,
            reporter,
        ).start());

        Self { tx }
    }

    pub async fn status(&self) {
        ask!(self.tx, ExperimentExecutorMsg::Status);
    }
}