use std::fmt;

use colored::Colorize;

use lib_interop::domain::DReport;
use lib_ui::*;

use crate::report::{ReportMessageWidget, ReportTypeWidget};

pub struct InlineReportWidget<'a> {
    report: &'a DReport,
}

impl<'a> InlineReportWidget<'a> {
    pub fn new(report: &'a DReport) -> Self {
        Self { report }
    }
}

impl fmt::Display for InlineReportWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let at = DateTimeWidget::new(self.report.at)
            .to_string()
            .dimmed();

        write!(
            f,
            "{} {} | {}",
            at,
            ReportTypeWidget::new(&self.report.ty),
            ReportMessageWidget::new(&self.report),
        )
    }
}