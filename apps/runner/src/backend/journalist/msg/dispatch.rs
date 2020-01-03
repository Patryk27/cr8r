use lib_interop::domain::DEvent;

use crate::backend::journalist::JournalistActor;

pub async fn dispatch(actor: &mut JournalistActor, event: DEvent) {
    actor.pending_events.push_back(event);

    while let Some(event) = actor.pending_events.pop_front() {
        if actor.client.add_event(event.clone().into()).await.is_err() {
            actor.pending_events.push_front(event);
            break;
        }
    }
}