use std::sync::Arc;

use lib_protocol::core::PReport;

use crate::backend::experiment_watcher::ExperimentWatcherActor;

pub fn process(actor: &mut ExperimentWatcherActor, report: Arc<PReport>) {
    if !actor.alive {
        return;
    }

    if let Some(reply) = ExperimentWatcherActor::report_to_reply(&report) {
        actor.add_pending_reply(reply);
    }
}