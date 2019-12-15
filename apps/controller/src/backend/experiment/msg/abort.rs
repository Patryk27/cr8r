use chrono::Utc;

use crate::backend::experiment::{ExperimentActor, ExperimentStatus};

pub fn process(actor: &mut ExperimentActor) {
    match &mut actor.status {
        ExperimentStatus::Running { .. } => {
//            if let Some(watcher) = &mut actor.watcher {
//                watcher.add_event(Arc::new(PReport {
//                    created_at: Utc::now().to_rfc3339(),
//                    op: Some(Op::ExperimentAborted(PExperimentAborted {})),
//                }));
//            }

            actor.status = ExperimentStatus::Aborted {
                since: Utc::now(),
            };
        }

        _ => (),
    }
}