use std::fmt;

use lib_interop::contract::{CReport, CReportType};

pub struct ReportMessage<'a> {
    report: &'a CReport,
}

impl<'a> ReportMessage<'a> {
    pub fn new(report: &'a CReport) -> Self {
        Self { report }
    }
}

impl fmt::Display for ReportMessage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        let msg = &self.report.msg;

        let msg = match self.report.ty {
            CReportType::SystemMsg => msg
                .blue()
                .to_string(),

            CReportType::UserMsg => msg
                .white()
                .to_string(),

            CReportType::ProcessOutput => msg
                .white()
                .dimmed()
                .to_string(),
        };

        write!(f, "{}", msg)
    }
}