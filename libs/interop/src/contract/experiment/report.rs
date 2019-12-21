use chrono::{DateTime, Utc};

use crate::protocol::core::PExperimentReport;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CExperimentReportKind {
    SystemMsg,
    UserMsg,
    ProcessOutput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CExperimentReport {
    created_at: DateTime<Utc>,
    kind: CExperimentReportKind,
    message: String,
}

impl Into<PExperimentReport> for &'_ CExperimentReport {
    fn into(self) -> PExperimentReport {
        unimplemented!()
    }
}