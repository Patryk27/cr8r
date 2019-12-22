use chrono::{DateTime, Utc};

use crate::protocol::core::p_report::Kind;
use crate::protocol::core::PReport;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CReportType {
    SystemMsg,
    UserMsg,
    ProcessOutput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CReport {
    at: DateTime<Utc>,
    ty: CReportType,
    msg: String,
}

impl CReport {
    pub fn system_msg(at: DateTime<Utc>, msg: impl Into<String>) -> Self {
        Self {
            at,
            ty: CReportType::SystemMsg,
            msg: msg.into(),
        }
    }

    pub fn user_msg(at: DateTime<Utc>, msg: impl Into<String>) -> Self {
        Self {
            at,
            ty: CReportType::UserMsg,
            msg: msg.into(),
        }
    }

    pub fn process_output(at: DateTime<Utc>, msg: impl Into<String>) -> Self {
        Self {
            at,
            ty: CReportType::ProcessOutput,
            msg: msg.into(),
        }
    }
}

impl Into<PReport> for &'_ CReport {
    fn into(self) -> PReport {
        let kind = match self.ty {
            CReportType::SystemMsg => Kind::SystemMsg,
            CReportType::UserMsg => Kind::UserMsg,
            CReportType::ProcessOutput => Kind::ProcessOutput,
        } as _;

        PReport {
            created_at: self.at.to_rfc3339(),
            kind,
            message: self.msg.to_owned(),
        }
    }
}