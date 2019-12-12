use futures_channel::{mpsc, oneshot};
use log::*;

use lib_actor::ActorSpirit;
use lib_protocol::core::PRunner;

use crate::backend::runner::RunnerActor;

pub type RunnerTx = mpsc::UnboundedSender<RunnerMsg>;
pub type RunnerRx = mpsc::UnboundedReceiver<RunnerMsg>;

#[derive(Debug)]
pub enum RunnerMsg {
    AsModel {
        tx: oneshot::Sender<PRunner>,
    },

    Kill,
}

mod as_model;

impl RunnerMsg {
    pub fn process(self, actor: &mut RunnerActor) -> ActorSpirit {
        debug!("Processing message: {:?}", self);

        match self {
            RunnerMsg::AsModel { tx } => {
                let _ = tx.send(as_model::process(actor));
                ActorSpirit::Alive
            }

            RunnerMsg::Kill => {
                ActorSpirit::Dead
            }
        }
    }
}