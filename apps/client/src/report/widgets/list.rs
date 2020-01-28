use std::fmt;

use colored::Colorize;
use prettytable::{cell, row};

use lib_interop::domain::DReport;
use lib_ui::*;

use crate::report::{ReportMessageWidget, ReportTypeWidget};

pub struct ReportListWidget<'a> {
    reports: &'a [DReport],
}

impl<'a> ReportListWidget<'a> {
    pub fn new(reports: &'a [DReport]) -> Self {
        Self { reports }
    }
}

impl fmt::Display for ReportListWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.reports.is_empty() {
            return writeln!(f, "There are no reports");
        }

        let mut table = table! {
            titles: ["▲ At", "Type", "Message"],
        };

        for report in self.reports {
            let at = DateTimeWidget::new(report.at)
                .to_string()
                .dimmed();

            table.add_row(row![
                at,
                ReportTypeWidget::new(&report.ty),
                ReportMessageWidget::new(report),
            ]);
        }

        write!(f, "{}", table)
    }
}
