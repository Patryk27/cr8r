use std::path::PathBuf;

use anyhow::*;
use tokio::sync::mpsc::unbounded_channel;

use lib_core_channel::{URx, UTx};
use lib_interop::proto::models::PAttachmentId;

use crate::modules::app::AppContext;

mod compress_dir;
mod upload;

// @todo use `AttachmentsClient` directly
pub struct AttachmentUploader<'c> {
    ctxt: &'c mut AppContext,
    progress: UTx<AttachmentUploaderProgress>,
}

pub enum AttachmentUploaderProgress {
    CompressingAttachment,

    AttachmentCompressed {
        total_bytes: u64,
    },

    UploadingAttachment {
        sent_bytes: u64,
    },

    AttachmentUploaded,
}

impl<'c> AttachmentUploader<'c> {
    pub fn new(ctxt: &'c mut AppContext) -> (Self, URx<AttachmentUploaderProgress>) {
        let (tx, rx) = unbounded_channel();

        (Self { ctxt, progress: tx }, rx)
    }

    /// Compresses specified directory and sends it into the Controller.
    pub async fn upload_dir(&mut self, path: impl Into<PathBuf>) -> Result<PAttachmentId> {
        let path = path.into();

        let archive = self
            .compress_dir(path)
            .await?;

        self.upload(archive.path_buf())
            .await
    }
}
