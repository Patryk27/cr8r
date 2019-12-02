use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use log::*;

use lib_protocol::core::{self, RunnerId, RunnerName};

use crate::backend::{RunnerCommand, RunnerCommandRx, System};

pub struct RunnerActor {
    system: System,
    id: RunnerId,
    name: RunnerName,
    joined_at: DateTime<Utc>,
    heartbeaten_at: DateTime<Utc>,
}

impl RunnerActor {
    pub fn new(system: System, id: RunnerId, name: RunnerName) -> Self {
        Self {
            system,
            id,
            name,
            joined_at: Utc::now(),
            heartbeaten_at: Utc::now(),
        }
    }

    pub async fn start(self, mut rx: RunnerCommandRx) {
        debug!("Actor started, entering event loop");
        debug!("-> id: {}", self.id);
        debug!("-> name: {}", self.name);

        while let Some(cmd) = rx.next().await {
            debug!("Processing command: {:?}", cmd);

            match cmd {
                RunnerCommand::AsModel { tx } => {
                    let _ = tx.send(
                        self.as_model(),
                    );
                }
            }
        }

        debug!("Actor orphaned, halting it");
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