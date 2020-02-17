use anyhow::*;
use derivative::Derivative;
use log::*;

use lib_core_channel::{OTx, SendTo};
use lib_interop::domain::{DRunnerId, DRunnerName};

use crate::system::Runner;

use super::RunnerStoreActor;

mod find_all;
mod register;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum RunnerStoreMsg {
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

impl RunnerStoreMsg {
    pub fn handle(self, actor: &mut RunnerStoreActor) {
        use RunnerStoreMsg::*;

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