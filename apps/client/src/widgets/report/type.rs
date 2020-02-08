use std::fmt;

use colored::Colorize;

use lib_interop::domain::DReportType;

pub struct ReportTypeWidget<'a> {
    ty: &'a DReportType,
}

impl<'a> ReportTypeWidget<'a> {
    pub fn new(ty: &'a DReportType) -> Self {
        Self { ty }
    }
}

impl fmt::Display for ReportTypeWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //@formatter:off
        let ty = match self.ty {
            DReportType::SystemMsg  => "sys ",
            DReportType::CustomMsg  => "msg ",
            DReportType::ProcessMsg => "proc",
        };
        //@formatter:on

        write!(f, "{}", ty.dimmed())
    }
}