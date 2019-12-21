use std::fmt;

use lib_interop::protocol::core::PReport;

pub struct InlineReport<'a> {
    report: &'a PReport,
}

impl<'a> InlineReport<'a> {
    pub fn new(report: &'a PReport) -> Self {
        Self { report }
    }
}

impl fmt::Display for InlineReport<'_> {
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
            ui::ReportKind::new(self.report),
            ui::ReportMessage::new(self.report),
        )
    }
}