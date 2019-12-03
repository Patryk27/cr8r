use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::{self, RunnerId, RunnerName};

use crate::backend::{RunnerMsg, RunnerRx, System};

pub struct RunnerActor {
    rx: RunnerRx,
    system: System,
    id: RunnerId,
    name: RunnerName,
    joined_at: DateTime<Utc>,
    heartbeaten_at: DateTime<Utc>,
}

impl RunnerActor {
    pub fn new(rx: RunnerRx, system: System, id: RunnerId, name: RunnerName) -> Self {
        Self {
            rx,
            system,
            id,
            name,
            joined_at: Utc::now(),
            heartbeaten_at: Utc::now(),
        }
    }

    pub async fn start(mut self) {
        debug!("Actor started, entering event loop");
        debug!("-> id: {}", self.id);
        debug!("-> name: {}", self.name);

        while let Some(msg) = self.rx.next().await {
            debug!("Processing message: {:?}", msg);

            match msg {
                RunnerMsg::AsModel { tx } => {
                    let _ = tx.send(self.as_model());
                }
            }
        }

        debug!("Actor orphaned, halting");
    }

    fn as_model(&self) -> core::Runner {
        use self::core::runner::status;

        // @todo provide actual status
        let status = status::Op::Idle(status::Idle {
            since: self.joined_at.to_rfc3339(),
        });

        core::Runner {
            id: self.id.clone(),
            name: self.name.clone(),
            joined_at: self.joined_at.to_rfc3339(),
            heartbeaten_at: self.heartbeaten_at.to_rfc3339(),

            status: Some(core::runner::Status {
                op: Some(status),
            }),
        }
    }
}