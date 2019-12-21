use std::fmt;

use lib_interop::protocol::core::PReport;

pub struct ReportsTable<'a> {
    reports: &'a [PReport],
}

impl<'a> ReportsTable<'a> {
    pub fn new(reports: &'a [PReport]) -> Self {
        Self { reports }
    }
}

impl fmt::Display for ReportsTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::{table, ui};
        use colored::Colorize;
        use prettytable::{cell, row};

        if self.reports.is_empty() {
            return write!(f, "There are no reports");
        }

        let mut table = table! {
            titles: ["Created at", "Kind", "Message"],
        };

        for report in self.reports {
            let created_at = ui::DateTime::new(&report.created_at)
                .to_string()
                .dimmed();

            table.add_row(row![
                created_at,
                ui::ReportKind::new(report),
                ui::ReportMessage::new(report),
            ]);
        }

        write!(f, "{}", table)
    }
}
