use std::collections::VecDeque;

use chrono::Utc;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::domain::{DEvent, DEventType, DExperimentId};

use crate::rpc::ControllerSession;

use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone)]
pub struct Logger {
    tx: UTx<LoggerMsg>,
}

impl Logger {
    pub fn new(session: ControllerSession, experiment_id: DExperimentId) -> Self {
        let (tx, rx) = unbounded_channel();

        spawn(LoggerActor {
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

        tell!(self.tx, LoggerMsg::Add { event });
    }
}