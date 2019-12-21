use log::*;
use tokio::sync::{mpsc, oneshot};

use lib_actor::ActorSpirit;
use lib_interop::contract::CRunner;

use crate::backend::runner::RunnerActor;

pub type RunnerTx = mpsc::UnboundedSender<RunnerMsg>;
pub type RunnerRx = mpsc::UnboundedReceiver<RunnerMsg>;

#[derive(Debug)]
pub enum RunnerMsg {
    GetModel {
        tx: oneshot::Sender<CRunner>,
    },

    Kill,
}

mod get_model;

impl RunnerMsg {
    pub fn handle(self, actor: &mut RunnerActor) -> ActorSpirit {
        debug!("Handling message: {:?}", self);

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