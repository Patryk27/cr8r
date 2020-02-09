use std::collections::VecDeque;

use chrono::Utc;
use tokio::{sync::mpsc, task};

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::domain::{DEvent, DEventType, DExperimentId};

use crate::session::Session;

use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone)]
pub struct ExperimentLogger {
    tx: UTx<ExperimentLoggerMsg>,
}

impl ExperimentLogger {
    pub fn new(session: Session, experiment_id: DExperimentId) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        task::spawn(ExperimentLoggerActor {
            session,
            experiment_id,
            pending_events: VecDeque::new(),
        }.start(rx));

        Self { tx }
    }

    pub fn add(&self, ty: DEventType) {
        let event = DEvent {
            at: Utc::now(),
            ty,
        };

        tell!(self.tx, ExperimentLoggerMsg::Add { event });
    }
}