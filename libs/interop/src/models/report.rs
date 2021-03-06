use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::conv;
use crate::models::{ModelError, ModelResult};
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

    pub fn process_msg(at: DateTime<Utc>, msg: impl Into<String>) -> Self {
        Self {
            at,
            ty: DReportType::ProcessMsg,
            msg: msg.into(),
        }
    }
}

impl TryFrom<PReport> for DReport {
    type Error = ModelError;

    fn try_from(PReport { at, ty, msg }: PReport) -> ModelResult<Self> {
        let ty = match PReportType::from_i32(ty).unwrap_or(PReportType::SystemMsg) {
            PReportType::SystemMsg => DReportType::SystemMsg,
            PReportType::ProcessMsg => DReportType::ProcessMsg,
        };

        Ok(Self {
            at: conv!(at as DateTime),
            ty,
            msg,
        })
    }
}

impl Into<PReport> for &'_ DReport {
    fn into(self) -> PReport {
        let ty = match self.ty {
            DReportType::SystemMsg => PReportType::SystemMsg,
            DReportType::ProcessMsg => PReportType::ProcessMsg,
        } as _;

        PReport {
            at: self.at.to_rfc3339(),
            ty,
            msg: self.msg.to_owned(),
        }
    }
}