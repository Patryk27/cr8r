use anyhow::*;
use derivative::Derivative;
use log::*;

use lib_core_channel::{OTx, SendTo};
use lib_interop::domain::{DRunnerId, DRunnerName};

use crate::system::Runner;

use super::RunnersActor;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum RunnersMsg {
    FindAll {
        #[derivative(Debug = "ignore")]
        tx: OTx<Vec<Runner>>,
    },

    Register {
        name: DRunnerName,

        #[derivative(Debug = "ignore")]
        tx: OTx<Result<DRunnerId>>,
    },
}

mod find_all;
mod register;

impl RunnersMsg {
    pub fn handle(self, actor: &mut RunnersActor) {
        use RunnersMsg::*;

        trace!("Handling message: {:?}", self);

        match self {
            FindAll { tx } => {
                find_all::find_all(actor)
                    .send_to(tx);
            }

            Register { name, tx } => {
                register::register(actor, name)
                    .send_to(tx);
            }
        }
    }
}