use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::convert;
use crate::domain::{DomainError, DomainResult};
use crate::proto::models::{PReport, PReportType};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DReport {
    pub at: DateTime<Utc>,
    pub ty: DReportType,
    pub msg: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DReportType {
    SystemMsg,
    CustomMsg,
    ProcessMsg,
}

impl DReport {
    pub fn system_msg(at: DateTime<Utc>, msg: impl Into<String>) -> Self {
        Self {
            at,
            ty: DReportType::SystemMsg,
            msg: msg.into(),
        }
    }

    pub fn custom_msg(at: DateTime<Utc>, msg: impl Into<String>) -> Self {
        Self {
            at,
            ty: DReportType::CustomMsg,
            msg: msg.into(),
        }
    }

    pub fn process_msg(at: DateTime<Utc>, msg: impl Into<String>) -> Self {
        Self {
            at,
            ty: DReportType::ProcessMsg,
            msg: msg.into(),
        }
    }
}

impl TryFrom<PReport> for DReport {
    type Error = DomainError;

    fn try_from(PReport { at, ty, msg }: PReport) -> DomainResult<Self> {
        let ty = match PReportType::from_i32(ty).unwrap_or(PReportType::CustomMsg) {
            PReportType::SystemMsg => DReportType::SystemMsg,
            PReportType::CustomMsg => DReportType::CustomMsg,
            PReportType::ProcessMsg => DReportType::ProcessMsg,
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
            DReportType::CustomMsg => PReportType::CustomMsg,
            DReportType::ProcessMsg => PReportType::ProcessMsg,
        } as _;

        PReport {
            at: self.at.to_rfc3339(),
            ty,
            msg: self.msg.to_owned(),
        }
    }
}