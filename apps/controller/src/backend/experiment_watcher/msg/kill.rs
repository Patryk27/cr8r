use chrono::Utc;
use log::*;

use lib_protocol::for_client::p_watch_experiment_reply::Kind;
use lib_protocol::for_client::PWatchExperimentReply;

use crate::backend::experiment_watcher::ExperimentWatcherActor;

pub fn process(actor: &mut ExperimentWatcherActor) {
    if !actor.alive {
        warn!("Tried to kill an already dead watcher - this may be a bug");
        return;
    }

    actor.add_pending_reply(PWatchExperimentReply {
        created_at: Utc::now().to_rfc3339(),
        kind: Kind::StreamClosed as i32,
        message: "".to_string(),
    });

    actor.alive = false;
}