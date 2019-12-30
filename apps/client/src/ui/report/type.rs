use std::fmt;

use lib_interop::contract::CReportType;

pub struct ReportType<'a> {
    ty: &'a CReportType,
}

impl<'a> ReportType<'a> {
    pub fn new(ty: &'a CReportType) -> Self {
        Self { ty }
    }
}

impl fmt::Display for ReportType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        //@formatter:off
        let ty = match self.ty {
            CReportType::SystemMsg     => "sys ",
            CReportType::UserMsg       => "msg ",
            CReportType::ProcessOutput => "proc",
        };
        //@formatter:on

        write!(f, "{}", ty.dimmed())
    }
}