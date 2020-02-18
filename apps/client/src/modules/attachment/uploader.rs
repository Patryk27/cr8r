use std::path::PathBuf;

use anyhow::*;
use tokio::sync::mpsc::unbounded_channel;

use lib_core_channel::{URx, UTx};
use lib_interop::clients::AttachmentClient;
use lib_interop::models::DAttachmentId;

mod compress_dir;
mod upload;

pub struct AttachmentUploader {
    client: AttachmentClient,
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

impl AttachmentUploader {
    pub fn new(client: AttachmentClient) -> (Self, URx<AttachmentUploaderProgress>) {
        let (progress, progress_rx) = unbounded_channel();

        (Self { client, progress }, progress_rx)
    }

    pub async fn upload_dir(&mut self, path: impl Into<PathBuf>) -> Result<DAttachmentId> {
        let path = path.into();

        let archive = self
            .compress_dir(path)
            .await?;

        self.upload(archive.path_buf())
            .await
    }
}
