use log::*;
use tokio::sync::{mpsc, oneshot};

use lib_actor::ActorSpirit;
use lib_protocol::core::PRunner;

use crate::backend::runner::RunnerActor;

pub type RunnerTx = mpsc::UnboundedSender<RunnerMsg>;
pub type RunnerRx = mpsc::UnboundedReceiver<RunnerMsg>;

#[derive(Debug)]
pub enum RunnerMsg {
    GetModel {
        tx: oneshot::Sender<PRunner>,
    },

    Kill,
}

mod get_model;

impl RunnerMsg {
    pub fn process(self, actor: &mut RunnerActor) -> ActorSpirit {
        debug!("Processing message: {:?}", self);

        match self {
            RunnerMsg::GetModel { tx } => {
                let _ = tx.send(get_model::get_model(actor));
                ActorSpirit::KeepAlive
            }

            RunnerMsg::Kill => {
                ActorSpirit::Kill
            }
        }
    }
}