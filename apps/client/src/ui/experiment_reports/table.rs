use std::fmt;

use lib_protocol::core::PExperimentReport;

pub struct ExperimentReportsTable<'a> {
    reports: &'a [PExperimentReport],
}

impl<'a> ExperimentReportsTable<'a> {
    pub fn new(reports: &'a [PExperimentReport]) -> Self {
        Self { reports }
    }
}

impl fmt::Display for ExperimentReportsTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::ui;
        use prettytable::*;

        if self.reports.is_empty() {
            return write!(f, "There are no reports");
        }

        let mut table = Table::new();

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        table.set_titles(row![
            "Created at", "Kind", "Message",
        ]);

        for report in self.reports {
            let at = ui::DateTime::new(&report.created_at);

            // @todo
        }

        write!(f, "{}", table)
    }
}
