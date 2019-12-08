use futures_channel::mpsc;

pub type ExperimentReporterTx = mpsc::UnboundedSender<ExperimentReporterMsg>;
pub type ExperimentReporterRx = mpsc::UnboundedReceiver<ExperimentReporterMsg>;

#[derive(Debug)]
pub enum ExperimentReporterMsg {
    ReportMessage {
        message: String,
    },

    ReportProcessStdout {
        line: String,
    },

    ReportProcessStderr {
        line: String,
    },

    ReportExperimentStarted,

    ReportExperimentCompleted,

    ReportScenarioStarted,

    ReportScenarioCompleted {
        success: bool,
    },
}