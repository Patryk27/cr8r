use futures_channel::mpsc;

pub type ExperimentJournalistTx = mpsc::UnboundedSender<ExperimentJournalistMsg>;
pub type ExperimentJournalistRx = mpsc::UnboundedReceiver<ExperimentJournalistMsg>;

#[derive(Debug)]
pub enum ExperimentJournalistMsg {
    AddSystemMsg {
        msg: String,
    },

    AddUserMsg {
        msg: String,
    },

    AddProcessOutput {
        line: String,
    },

    AddExperimentStarted,

    AddExperimentSucceeded,

    AddExperimentFailed {
        cause: String,
    },

    AddStepSucceeded {
        id: u32,
    },

    AddStepFailed {
        id: u32,
        cause: String,
    },
}