use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::models::DExperimentId;
use lib_sandbox::Sandbox;

use crate::rpc::Session;
use crate::system::{AttachmentStore, Logger};

use self::{
    actor::*,
    context::*,
    msg::*,
};
pub use self::status::*;

mod actor;
mod context;
mod msg;
mod status;

pub struct Executor {
    tx: UTx<ExecutorMsg>,
}

impl Executor {
    pub fn new(
        attachment_store: AttachmentStore,
        session: Session,
        sandbox: Sandbox,
        logger: Logger,
        experiment_id: DExperimentId,
    ) -> Self {
        let (tx, mailbox) = unbounded_channel();

        spawn(ExecutorActor {
            attachment_store,
            session,
            sandbox,
            logger,
            experiment_id,
            status: ExecutorStatus::default(),
            mailbox,
        }.start());

        Self { tx }
    }

    pub async fn get_status(&self) -> ExecutorStatus {
        ask!(self.tx, ExecutorMsg::GetStatus)
    }

    pub async fn stop(&self) {
        tell!(self.tx, ExecutorMsg::Stop);
    }
}