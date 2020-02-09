use std::sync::Arc;

use lib_interop::domain::DReport;

use super::super::{ExperimentActor, ExperimentStatus};

pub fn get_reports(actor: &ExperimentActor) -> Vec<Arc<DReport>> {
    match &actor.status {
        ExperimentStatus::Running { reports, .. } | ExperimentStatus::Completed { reports, .. } => {
            reports.to_vec()
        }

        _ => Vec::new(),
    }
}