use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::{convert, Error, Result};
use crate::protocol::core::{PReport, PReportType};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CReportType {
    SystemMsg,
    UserMsg,
    ProcessOutput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CReport {
    pub at: DateTime<Utc>,
    pub ty: CReportType,
    pub msg: String,
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

impl TryFrom<PReport> for CReport {
    type Error = Error;

    fn try_from(PReport { at, ty, msg }: PReport) -> Result<Self> {
        let ty = match PReportType::from_i32(ty).unwrap_or(PReportType::UserMsg) {
            PReportType::SystemMsg => CReportType::SystemMsg,
            PReportType::UserMsg => CReportType::UserMsg,
            PReportType::ProcessOutput => CReportType::ProcessOutput,
        };

        Ok(Self {
            at: convert!(at as DateTime),
            ty,
            msg,
        })
    }
}

impl Into<PReport> for &'_ CReport {
    fn into(self) -> PReport {
        let ty = match self.ty {
            CReportType::SystemMsg => PReportType::SystemMsg,
            CReportType::UserMsg => PReportType::UserMsg,
            CReportType::ProcessOutput => PReportType::ProcessOutput,
        } as _;

        PReport {
            at: self.at.to_rfc3339(),
            ty,
            msg: self.msg.to_owned(),
        }
    }
}