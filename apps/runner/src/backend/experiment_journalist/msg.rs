use futures_channel::mpsc;

pub type ExperimentJournalistTx = mpsc::UnboundedSender<ExperimentJournalistMsg>;
pub type ExperimentJournalistRx = mpsc::UnboundedReceiver<ExperimentJournalistMsg>;

#[derive(Debug)]
pub enum ExperimentJournalistMsg {
    AddCustomMessage {
        message: String,
    },

    AddProcessOutput {
        line: String,
    },

    AddExperimentStarted,

    AddExperimentCompleted,

    AddScenarioStarted,

    AddScenarioCompleted {
        success: bool,
    },
}