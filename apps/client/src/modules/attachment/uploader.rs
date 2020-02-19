use std::path::PathBuf;

use anyhow::*;
use tokio::sync::mpsc::unbounded_channel;

use lib_core_channel::{URx, UTx};
use lib_interop::clients::AttachmentClient;
use lib_interop::models::DAttachmentId;

mod compress_dir;
mod upload_archive;

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

    pub async fn upload_dir(&mut self, dir: impl Into<PathBuf>) -> Result<DAttachmentId> {
        let archive = self.compress_dir(dir.into()).await?;

        self.upload_archive(archive.path_buf()).await
    }
}
