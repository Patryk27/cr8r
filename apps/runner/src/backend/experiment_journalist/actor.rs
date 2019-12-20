use log::*;
use tokio::stream::StreamExt;

use lib_protocol::core::p_experiment_event::*;

use crate::backend::experiment_journalist::{ExperimentJournalistMsg, ExperimentJournalistRx};
use crate::core::ExperimentClient;

pub struct ExperimentJournalistActor {
    rx: ExperimentJournalistRx,
    client: ExperimentClient,
}

impl ExperimentJournalistActor {
    pub fn new(rx: ExperimentJournalistRx, client: ExperimentClient) -> Self {
        Self { rx, client }
    }

    pub async fn main(mut self) {
        debug!("Actor started");

        while let Some(msg) = self.rx.next().await {
            debug!("Processing message: {:?}", msg);

            let report = Self::msg_to_report(msg);

            if let Err(err) = self.client.add_event(report).await {
                error!("Couldn't send report to the controller: {:?}", err);
                // @todo try again in a moment
            }
        }

        debug!("Actor orphaned, halting");
    }

    fn msg_to_report(msg: ExperimentJournalistMsg) -> Op {
        use ExperimentJournalistMsg::*;
        use Op::*;

        match msg {
            AddSystemMsg { msg } => {
                SystemMsg(PSystemMsg { msg })
            }

            AddUserMsg { msg } => {
                UserMsg(PUserMsg { msg })
            }

            AddProcessOutput { line } => {
                ProcessOutput(PProcessOutput { line })
            }

            AddExperimentStarted => {
                ExperimentStarted(PExperimentStarted {})
            }

            AddExperimentSucceeded => {
                ExperimentSucceeded(PExperimentSucceeded {})
            }

            AddExperimentFailed { cause } => {
                ExperimentFailed(PExperimentFailed { cause })
            }

            AddStepSucceeded { id } => {
                StepSucceeded(PStepSucceeded { id })
            }

            AddStepFailed { id, cause } => {
                StepFailed(PStepFailed { id, cause })
            }
        }
    }
}