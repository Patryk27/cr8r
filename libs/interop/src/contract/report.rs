use chrono::{DateTime, Utc};

use crate::protocol::core::PReport;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CReportKind {
    SystemMsg,
    UserMsg,
    ProcessOutput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CReport {
    created_at: DateTime<Utc>,
    kind: CReportKind,
    message: String,
}

impl CReport {
    pub fn system_msg(msg: impl Into<String>) -> Self {
        Self {
            created_at: Utc::now(),
            kind: CReportKind::SystemMsg,
            message: msg.into(),
        }
    }

    pub fn user_msg(msg: impl Into<String>) -> Self {
        Self {
            created_at: Utc::now(),
            kind: CReportKind::UserMsg,
            message: msg.into(),
        }
    }

    pub fn process_output(msg: impl Into<String>) -> Self {
        Self {
            created_at: Utc::now(),
            kind: CReportKind::ProcessOutput,
            message: msg.into(),
        }
    }
}

impl Into<PReport> for &'_ CReport {
    fn into(self) -> PReport {
        unimplemented!()
    }
}