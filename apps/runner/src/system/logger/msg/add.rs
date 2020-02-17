use lib_interop::domain::DEvent;
use lib_interop::proto::services::PAddEventRequest;

use super::super::LoggerActor;

pub async fn add(
    LoggerActor { session, experiment_id, pending_events, .. }: &mut LoggerActor,
    event: DEvent,
) {
    pending_events.push_back(event);

    while let Some(event) = pending_events.pop_front() {
        let reply = session.conn
            .events()
            .add_event(PAddEventRequest {
                runner_id: session.runner_id,
                experiment_id: experiment_id.as_num(),
                event: Some(event.clone().into()),
            })
            .await;

        if reply.is_err() {
            pending_events.push_front(event);
            break;
        }
    }
}