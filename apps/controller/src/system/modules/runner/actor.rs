use chrono::{DateTime, Utc};
use log::*;
use tokio::select;
use tokio::stream::StreamExt;
use tokio::time::{delay_for, Duration};

use lib_core_actor::ActorWorkflow;
use lib_core_channel::URx;
use lib_interop::models::{DRunnerId, DRunnerName};

use crate::system::{SystemEvent, SystemEventBus};

use super::{RunnerMsg, RunnerStatus};

pub struct RunnerActor {
    pub bus: SystemEventBus,
    pub id: DRunnerId,
    pub name: DRunnerName,
    pub joined_at: DateTime<Utc>,
    pub last_heartbeat_at: DateTime<Utc>,
    pub status: RunnerStatus,
}

impl RunnerActor {
    pub async fn start(mut self, mut mailbox: URx<RunnerMsg>) {
        trace!("Actor started");
        trace!("-> id = {}", self.id);
        trace!("-> name = {}", self.name);

        loop {
            self.triage();

            select! {
                msg = mailbox.next() => {
                    if self.process_message(msg).actor_should_stop() {
                        break;
                    }
                }

                _ = delay_for(Duration::from_secs(60)) => {
                    continue;
                }
            }
        }

        self.bus.emit(SystemEvent::RunnerLeft {
            id: self.id,
        });

        trace!("Actor halted");
    }

    fn triage(&mut self) {
        if !self.status.is_zombie() {
            if (Utc::now() - self.last_heartbeat_at).num_seconds() > 10 {
                warn!("Runner [id={}] has been running for some time without emitting a heartbeat", self.id);
                warn!(".. this runner is now officially considered a zombie");

                self.bus.emit(SystemEvent::RunnerTurnedZombie {
                    id: self.id,
                });

                take_mut::take(&mut self.status, |status| {
                    RunnerStatus::Zombie {
                        since: Utc::now(),
                        previous_status: box status,
                    }
                });
            }
        }
    }

    fn process_message(&mut self, msg: Option<RunnerMsg>) -> ActorWorkflow {
        if let Some(msg) = msg {
            msg.handle(self)
        } else {
            ActorWorkflow::Stop
        }
    }
}