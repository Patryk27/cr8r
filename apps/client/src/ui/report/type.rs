use std::fmt;

use lib_interop::domain::DReportType;

pub struct ReportType<'a> {
    ty: &'a DReportType,
}

impl<'a> ReportType<'a> {
    pub fn new(ty: &'a DReportType) -> Self {
        Self { ty }
    }
}

impl fmt::Display for ReportType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        //@formatter:off
        let ty = match self.ty {
            DReportType::SystemMsg     => "sys ",
            DReportType::UserMsg       => "msg ",
            DReportType::ProcessOutput => "proc",
        };
        //@formatter:on

        write!(f, "{}", ty.dimmed())
    }
}