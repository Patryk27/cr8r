use futures_channel::oneshot;

use lib_protocol::for_client::PWatchExperimentReply;

use crate::backend::experiment_watcher::ExperimentWatcherActor;

pub fn process(actor: &mut ExperimentWatcherActor, tx: oneshot::Sender<PWatchExperimentReply>) {
    if let Some(reply) = actor.pending_replies.pop_front() {
        let _ = tx.send(reply);
    } else {
        actor.pending_tx = Some(tx);
    }
}