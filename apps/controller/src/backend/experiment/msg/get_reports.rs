use std::sync::Arc;

use lib_interop::contract::CReport;

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};

pub fn get_reports(actor: &ExperimentActor) -> Vec<Arc<CReport>> {
    match &actor.status {
        ExperimentStatus::Running { reports, .. } | ExperimentStatus::Completed { reports, .. } => {
            reports.to_vec()
        }

        _ => Vec::new(),
    }
}