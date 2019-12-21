use tokio::sync::mpsc;

use lib_actor::tell;

use crate::core::ExperimentClient;

pub(self) use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone)]
pub struct ExperimentJournalist {
    tx: ExperimentJournalistTx,
}

impl ExperimentJournalist {
    pub fn new(client: ExperimentClient) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(ExperimentJournalistActor::new(
            rx,
            client,
        ).main());

        Self { tx }
    }

    pub fn add_user_msg(&self, msg: impl Into<String>) {
        tell!(self.tx, ExperimentJournalistMsg::AddUserMsg { msg: msg.into() });
    }

    pub fn add_system_msg(&self, msg: impl Into<String>) {
        tell!(self.tx, ExperimentJournalistMsg::AddSystemMsg { msg: msg.into() });
    }

    pub fn add_process_output(&self, line: impl Into<String>) {
        tell!(self.tx, ExperimentJournalistMsg::AddProcessOutput { line: line.into() });
    }

    pub fn add_experiment_started(&self) {
        tell!(self.tx, ExperimentJournalistMsg::AddExperimentStarted);
    }

    pub fn add_experiment_succeeded(&self) {
        tell!(self.tx, ExperimentJournalistMsg::AddExperimentSucceeded);
    }

    pub fn add_experiment_failed(&self, cause: impl Into<String>) {
        tell!(self.tx, ExperimentJournalistMsg::AddExperimentFailed { cause: cause.into() });
    }

    pub fn add_step_succeeded(&self, id: u32) {
        tell!(self.tx, ExperimentJournalistMsg::AddStepSucceeded { id });
    }

    pub fn add_step_failed(&self, id: u32, cause: impl Into<String>) {
        tell!(self.tx, ExperimentJournalistMsg::AddStepFailed { id, cause: cause.into() });
    }
}