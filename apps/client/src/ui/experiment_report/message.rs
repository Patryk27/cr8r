use std::fmt;

use lib_protocol::core::PExperimentReport;

pub struct ExperimentReportMessage<'a> {
    report: &'a PExperimentReport,
}

impl<'a> ExperimentReportMessage<'a> {
    pub fn new(report: &'a PExperimentReport) -> Self {
        Self { report }
    }
}

impl fmt::Display for ExperimentReportMessage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;
        use lib_protocol::core::p_experiment_report::Kind;

        let msg = &self.report.message;

        let msg = match Kind::from_i32(self.report.kind).unwrap_or(Kind::UserMsg) {
            Kind::SystemMsg => msg
                .blue()
                .to_string(),

            Kind::UserMsg => msg
                .white()
                .to_string(),

            Kind::ProcessOutput => msg
                .white()
                .dimmed()
                .to_string(),
        };

        write!(f, "{}", msg)
    }
}