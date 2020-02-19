use std::collections::BTreeMap;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use log::*;
use tokio::select;
use tokio::stream::StreamExt;
use tokio::time::{delay_for, Duration};

use lib_core_actor::ActorWorkflow;
use lib_core_channel::{URx, UTx};
use lib_interop::models::{DExperimentId, DJob, DReport};
use lib_interop::models::job::DJobId;

use crate::system::{Attachment, SystemEvent, SystemEventBus};

use super::{ExperimentMsg, ExperimentStatus};

pub struct ExperimentActor {
    pub bus: SystemEventBus,
    pub id: DExperimentId,
    pub attachments: Vec<Attachment>,
    pub jobs: BTreeMap<DJobId, DJob>,
    pub created_at: DateTime<Utc>,
    pub watchers: Vec<UTx<Arc<DReport>>>,
    pub status: ExperimentStatus,
}

impl ExperimentActor {
    pub async fn start(mut self, mut mailbox: URx<ExperimentMsg>) {
        trace!("Actor started");
        trace!("-> id = {}", self.id);

        loop {
            self.triage();

            select! {
                evt = self.bus.recv() => {
                    self.process_event(evt);
                }

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

        trace!("Actor halted");
    }

    fn triage(&mut self) {
        // @todo we have to implement heartbeating inside runner first

        // if let ExperimentStatus::Running { last_heartbeat_at, .. } = self.status {
        //     if (Utc::now() - last_heartbeat_at).num_minutes() >= 10 {
        //         warn!("Experiment [id={}] has been running for some time without hearing heartbeat from its runner", self.id);
        //         warn!(".. this experiment will get stopped");
        //
        //         ExperimentMsg::Stop.handle(self);
        //     }
        // }
    }

    fn process_event(&mut self, evt: SystemEvent) {
        match evt {
            SystemEvent::RunnerLeft { id } => {
                if let ExperimentStatus::Running { runner_id, .. } = self.status {
                    if runner_id == id {
                        warn!("Experiment [id={}] has been abandoned by its runner", self.id);
                        warn!(".. this experiment will get stopped");

                        ExperimentMsg::Stop.handle(self);
                    }
                }
            }
            SystemEvent::RunnerTurnedZombie { id } => {
                if let ExperimentStatus::Running { runner_id, .. } = self.status {
                    if runner_id == id {
                        warn!("Experiment [id={}]'s runner has turned into zombie", self.id);
                        warn!(".. this experiment will get stopped");

                        ExperimentMsg::Stop.handle(self);
                    }
                }
            }

            _ => (),
        }
    }

    fn process_message(&mut self, msg: Option<ExperimentMsg>) -> ActorWorkflow {
        if let Some(msg) = msg {
            msg.handle(self);
            ActorWorkflow::Continue
        } else {
            ActorWorkflow::Stop
        }
    }
}