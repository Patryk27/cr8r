use lib_protocol::core::PReport;

use crate::backend::experiment_watcher::ExperimentWatcherActor;

pub fn process(actor: &mut ExperimentWatcherActor, report: PReport) {
    if !actor.alive {
        return;
    }

    if let Some(tx) = actor.pending_get_tx.take() {
        let _ = tx.send(Some(
            ExperimentWatcherActor::render_report(report)
        ));
    } else {
        actor.reports.push_back(report);
    }
}