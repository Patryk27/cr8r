use std::fmt;

use lib_interop::domain::DReport;

pub struct ReportsTable<'a> {
    reports: &'a [DReport],
}

impl<'a> ReportsTable<'a> {
    pub fn new(reports: &'a [DReport]) -> Self {
        Self { reports }
    }
}

impl fmt::Display for ReportsTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::{table, ui};
        use colored::Colorize;
        use prettytable::{cell, row};

        if self.reports.is_empty() {
            return writeln!(f, "There are no reports");
        }

        let mut table = table! {
            titles: ["▲ At", "Type", "Message"],
        };

        for report in self.reports {
            let at = ui::DateTime::new(report.at)
                .to_string()
                .dimmed();

            table.add_row(row![
                at,
                ui::ReportType::new(&report.ty),
                ui::ReportMessage::new(report),
            ]);
        }

        write!(f, "{}", table)
    }
}
