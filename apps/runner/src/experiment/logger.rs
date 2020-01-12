use std::collections::VecDeque;

use chrono::Utc;
use tokio::sync::mpsc;

use lib_actor::tell;
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
    tx: ExperimentLoggerTx,
}

impl ExperimentLogger {
    pub fn new(session: Session, experiment_id: DExperimentId) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(ExperimentLoggerActor {
            rx,
            session,
            experiment_id,
            pending_events: VecDeque::new(),
        }.start());

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