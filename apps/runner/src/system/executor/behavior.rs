use std::collections::HashMap;

use lib_interop::models::{DAttachmentId, DJob};
use lib_sandbox::Sandbox;

use crate::system::{Attachment, AttachmentStore};

use super::ExecutorActor;

mod completed;
mod initializing;
mod running;
mod stopped;

pub enum ExecutorBehavior {
    Initializing {
        attachment_store: AttachmentStore,
        sandbox: Sandbox,
    },

    Running {
        sandbox: Sandbox,
        attachments: HashMap<DAttachmentId, Attachment>,
        jobs: Vec<DJob>,
    },

    Completed,

    Stopped,
}

impl ExecutorBehavior {
    pub async fn tick(self, actor: &mut ExecutorActor) -> Option<Self> {
        unimplemented!()
    }
}

