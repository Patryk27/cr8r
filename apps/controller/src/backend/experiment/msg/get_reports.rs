use std::sync::Arc;

use lib_interop::contract::CExperimentReport;

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};

pub fn get_reports(actor: &ExperimentActor) -> Vec<Arc<CExperimentReport>> {
    match &actor.status {
        ExperimentStatus::Running { reports, .. } | ExperimentStatus::Completed { reports, .. } => {
            reports.to_vec()
        }

        _ => Vec::new(),
    }
}