use std::fmt;

use colored::Colorize;

use lib_interop::domain::{DReport, DReportType};

pub struct ReportMessageWidget<'a> {
    report: &'a DReport,
}

impl<'a> ReportMessageWidget<'a> {
    pub fn new(report: &'a DReport) -> Self {
        Self { report }
    }
}

impl fmt::Display for ReportMessageWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = &self.report.msg;

        let msg = match self.report.ty {
            DReportType::SystemMsg => msg
                .blue()
                .to_string(),

            DReportType::CustomMsg => msg
                .white()
                .to_string(),

            DReportType::ProcessMsg => msg
                .white()
                .dimmed()
                .to_string(),
        };

        write!(f, "{}", msg)
    }
}