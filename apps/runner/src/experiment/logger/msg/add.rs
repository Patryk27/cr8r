use lib_interop::domain::DEvent;

use super::super::ExperimentLoggerActor;

pub async fn add(
    ExperimentLoggerActor { session, experiment_id, pending_events, .. }: &mut ExperimentLoggerActor,
    event: DEvent,
) {
    pending_events.push_back(event);

    while let Some(event) = pending_events.pop_front() {
        let result = session.invoke(|client, runner_id| {
            client.add_event(
                runner_id,
                experiment_id.as_num(),
                event.clone().into(),
            )
        }).await;

        if result.is_err() {
            pending_events.push_front(event);
            break;
        }
    }
}