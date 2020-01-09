use std::fmt;

use lib_interop::domain::{DReport, DReportType};

pub struct ReportMessage<'a> {
    report: &'a DReport,
}

impl<'a> ReportMessage<'a> {
    pub fn new(report: &'a DReport) -> Self {
        Self { report }
    }
}

impl fmt::Display for ReportMessage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

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