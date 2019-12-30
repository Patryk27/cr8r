use std::fmt;

use lib_interop::contract::CReport;

pub struct InlineReport<'a> {
    report: &'a CReport,
}

impl<'a> InlineReport<'a> {
    pub fn new(report: &'a CReport) -> Self {
        Self { report }
    }
}

impl fmt::Display for InlineReport<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::ui;
        use colored::Colorize;

        let at = ui::DateTime::new(self.report.at)
            .to_string()
            .dimmed();

        write!(
            f,
            "{} {} | {}",
            at,
            ui::ReportType::new(&self.report.ty),
            ui::ReportMessage::new(&self.report),
        )
    }
}