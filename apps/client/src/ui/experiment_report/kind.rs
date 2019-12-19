use std::fmt;

use lib_protocol::core::PExperimentReport;

pub struct ExperimentReportKind<'a> {
    report: &'a PExperimentReport,
}

impl<'a> ExperimentReportKind<'a> {
    pub fn new(report: &'a PExperimentReport) -> Self {
        Self { report }
    }
}

impl fmt::Display for ExperimentReportKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;
        use lib_protocol::core::p_experiment_report::Kind;

        //@formatter:off
        let kind = match Kind::from_i32(self.report.kind).unwrap_or(Kind::UserMsg) {
            Kind::SystemMsg     => "sys ",
            Kind::UserMsg       => "msg ",
            Kind::ProcessOutput => "proc",
        };
        //@formatter:on

        write!(f, "{}", kind.dimmed())
    }
}