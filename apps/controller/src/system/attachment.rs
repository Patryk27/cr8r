use std::path::PathBuf;

use anyhow::*;
use chrono::Utc;
use tokio::fs::File;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::spawn;

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::models::{DAttachmentId, DAttachmentName};
use lib_interop::models::attachment::DAttachment;
use lib_interop::proto::models::PAttachmentSize;

use self::{
    actor::*,
    msg::*,
    status::*,
};

mod actor;
mod msg;
mod status;

#[derive(Clone)]
pub struct Attachment {
    tx: UTx<AttachmentMsg>,
}

impl Attachment {
    pub fn new(id: DAttachmentId, name: DAttachmentName, size: PAttachmentSize, path: PathBuf, file: File) -> Self {
        let (tx, rx) = unbounded_channel();

        spawn(AttachmentActor {
            id,
            name,
            size,
            created_at: Utc::now(),
            status: Default::default(),
        }.start(path, file, rx));

        Self { tx }
    }

    pub async fn add_chunk(&self, chunk: Vec<u8>) -> Result<()> {
        ask!(self.tx, AttachmentMsg::AddChunk { chunk })
    }

    pub async fn commit(&self) -> Result<()> {
        ask!(self.tx, AttachmentMsg::Commit)
    }

    pub async fn get_model(&self) -> DAttachment {
        ask!(self.tx, AttachmentMsg::GetModel)
    }

    pub fn kill(&self) {
        tell!(self.tx, AttachmentMsg::Kill)
    }
}

#[cfg(test)]
mod tests {
    use lib_core_tempfile::TempFile;

    use super::*;

    mod get_name {
        use super::*;

        #[tokio::test]
        async fn returns_attachment_name() {
            let (attachment, file) = attachment().await;

            assert_eq!(
                DAttachmentName::from("winrar.rar"),
                attachment.get_name().await,
            );
        }
    }

    mod get_size {
        use super::*;

        #[tokio::test]
        async fn returns_attachment_size() {
            let (attachment, file) = attachment().await;

            assert_eq!(
                4096,
                attachment.get_size().await,
            );
        }
    }

    async fn attachment() -> (Attachment, TempFile) {
        let file = TempFile::new()
            .await
            .unwrap();

        let attachment = Attachment::new(
            100.into(),
            "winrar.rar".into(),
            4096,
            file.path_buf(),
            file.tokio_file().await.unwrap(),
        );

        (attachment, file)
    }
}