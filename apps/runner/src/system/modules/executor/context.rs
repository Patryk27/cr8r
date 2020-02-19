use std::collections::{HashMap, VecDeque};

use lib_interop::models::{DAttachmentId, DJob};

use crate::system::Attachment;

pub struct ExecutorContext {
    pub attachments: HashMap<DAttachmentId, Attachment>,
    pub jobs: VecDeque<DJob>,
}