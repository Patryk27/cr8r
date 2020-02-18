use lib_interop::models::DEvent;

use super::super::LoggerActor;

pub async fn add(
    LoggerActor { session, experiment_id, pending_events, .. }: &mut LoggerActor,
    event: DEvent,
) {
    pending_events.push_back(event);

    while let Some(event) = pending_events.pop_front() {
        let reply = session
            .conn()
            .events()
            .add(session.runner_id(), *experiment_id, event.clone())
            .await;

        if reply.is_err() {
            pending_events.push_front(event);
            break;
        }
    }
}