use std::fmt;

use lib_interop::protocol::core::PReport;

pub struct ReportKind<'a> {
    report: &'a PReport,
}

impl<'a> ReportKind<'a> {
    pub fn new(report: &'a PReport) -> Self {
        Self { report }
    }
}

impl fmt::Display for ReportKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;
        use lib_interop::protocol::core::p_report::Kind;

        //@formatter:off
        let kind = match Kind::from_i32(self.report.kind).unwrap_or(Kind::UserMsg) {
            Kind::SystemMsg     => "sys ",
            Kind::UserMsg       => "msg ",
            Kind::ProcessOutput => "proc",
        };
        //@formatter:on

        write!(f, "{}", kind.dimmed())
    }
}