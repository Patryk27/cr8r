use futures_channel::oneshot;

use crate::backend::experiment_watcher::ExperimentWatcherActor;

pub fn process(actor: &mut ExperimentWatcherActor, tx: oneshot::Sender<Option<String>>) {
    if let Some(report) = actor.reports.pop_front() {
        let _ = tx.send(Some(
            ExperimentWatcherActor::render_report(report)
        ));
    } else {
        actor.pending_get_tx = Some(tx);
    }
}