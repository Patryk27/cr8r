use std::fmt;

use lib_interop::protocol::core::PReport;

pub struct ReportMessage<'a> {
    report: &'a PReport,
}

impl<'a> ReportMessage<'a> {
    pub fn new(report: &'a PReport) -> Self {
        Self { report }
    }
}

impl fmt::Display for ReportMessage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;
        use lib_interop::protocol::core::p_report::Kind;

        let msg = &self.report.message;

        let msg = match Kind::from_i32(self.report.kind).unwrap_or(Kind::UserMsg) {
            Kind::SystemMsg => msg
                .blue()
                .to_string(),

            Kind::UserMsg => msg
                .white()
                .to_string(),

            Kind::ProcessOutput => msg
                .white()
                .dimmed()
                .to_string(),
        };

        write!(f, "{}", msg)
    }
}