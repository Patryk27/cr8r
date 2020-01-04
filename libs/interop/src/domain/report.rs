use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::convert;
use crate::domain::{DomainError, DomainResult};
use crate::proto::core::{PReport, PReportType};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DReportType {
    SystemMsg,
    UserMsg,
    ProcessOutput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DReport {
    pub at: DateTime<Utc>,
    pub ty: DReportType,
    pub msg: String,
}

impl DReport {
    pub fn system_msg(at: DateTime<Utc>, msg: impl Into<String>) -> Self {
        Self {
            at,
            ty: DReportType::SystemMsg,
            msg: msg.into(),
        }
    }

    pub fn user_msg(at: DateTime<Utc>, msg: impl Into<String>) -> Self {
        Self {
            at,
            ty: DReportType::UserMsg,
            msg: msg.into(),
        }
    }

    pub fn process_output(at: DateTime<Utc>, msg: impl Into<String>) -> Self {
        Self {
            at,
            ty: DReportType::ProcessOutput,
            msg: msg.into(),
        }
    }
}

impl TryFrom<PReport> for DReport {
    type Error = DomainError;

    fn try_from(PReport { at, ty, msg }: PReport) -> DomainResult<Self> {
        let ty = match PReportType::from_i32(ty).unwrap_or(PReportType::UserMsg) {
            PReportType::SystemMsg => DReportType::SystemMsg,
            PReportType::UserMsg => DReportType::UserMsg,
            PReportType::ProcessOutput => DReportType::ProcessOutput,
        };

        Ok(Self {
            at: convert!(at as DateTime),
            ty,
            msg,
        })
    }
}

impl Into<PReport> for &'_ DReport {
    fn into(self) -> PReport {
        let ty = match self.ty {
            DReportType::SystemMsg => PReportType::SystemMsg,
            DReportType::UserMsg => PReportType::UserMsg,
            DReportType::ProcessOutput => PReportType::ProcessOutput,
        } as _;

        PReport {
            at: self.at.to_rfc3339(),
            ty,
            msg: self.msg.to_owned(),
        }
    }
}