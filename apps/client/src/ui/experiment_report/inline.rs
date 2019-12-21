use std::fmt;

use lib_interop::protocol::core::PExperimentReport;

pub struct InlineExperimentReport<'a> {
    report: &'a PExperimentReport,
}

impl<'a> InlineExperimentReport<'a> {
    pub fn new(report: &'a PExperimentReport) -> Self {
        Self { report }
    }
}

impl fmt::Display for InlineExperimentReport<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::ui;
        use colored::Colorize;

        let created_at = ui::DateTime::new(&self.report.created_at)
            .to_string()
            .dimmed();

        write!(
            f,
            "{} {} | {}",
            created_at,
            ui::ExperimentReportKind::new(self.report),
            ui::ExperimentReportMessage::new(self.report),
        )
    }
}