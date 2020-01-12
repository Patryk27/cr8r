use derivative::Derivative;
use log::*;
use tokio::sync::{mpsc, oneshot};

use lib_actor::ActorWorkflow;
use lib_interop::domain::DRunner;

use crate::backend::runner::RunnerActor;

pub type RunnerTx = mpsc::UnboundedSender<RunnerMsg>;
pub type RunnerRx = mpsc::UnboundedReceiver<RunnerMsg>;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum RunnerMsg {
    GetModel {
        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<DRunner>,
    },

    Kill,
}

mod get_model;

impl RunnerMsg {
    pub fn handle(self, actor: &mut RunnerActor) -> ActorWorkflow {
        debug!("Handling message: {:?}", self);

        match self {
            RunnerMsg::GetModel { tx } => {
                let _ = tx.send(get_model::get_model(actor));
                ActorWorkflow::Continue
            }

            RunnerMsg::Kill => {
                ActorWorkflow::Stop
            }
        }
    }
}