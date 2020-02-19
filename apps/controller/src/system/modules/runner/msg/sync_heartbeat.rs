use chrono::Utc;

use crate::system::SystemEvent;

use super::super::{RunnerActor, RunnerStatus};

pub fn sync_heartbeat(actor: &mut RunnerActor) {
    if actor.status.is_zombie() {
        actor.bus.emit(SystemEvent::RunnerTurnedAlive {
            id: actor.id,
        });

        take_mut::take(&mut actor.status, |status| {
            if let RunnerStatus::Zombie { previous_status, .. } = status {
                *previous_status
            } else {
                status
            }
        });
    }

    actor.last_heartbeat_at = Utc::now();
}