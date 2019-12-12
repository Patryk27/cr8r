use futures_channel::mpsc;

pub type ExperimentReporterTx = mpsc::UnboundedSender<ExperimentReporterMsg>;
pub type ExperimentReporterRx = mpsc::UnboundedReceiver<ExperimentReporterMsg>;

#[derive(Debug)]
pub enum ExperimentReporterMsg {
    AddMessage {
        message: String,
    },

    AddProcessStdout {
        line: String,
    },

    AddProcessStderr {
        line: String,
    },

    AddExperimentStarted,

    AddExperimentCompleted,

    AddScenarioStarted,

    AddScenarioCompleted {
        success: bool,
    },
}