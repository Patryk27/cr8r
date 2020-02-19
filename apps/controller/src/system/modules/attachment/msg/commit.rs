use std::sync::Arc;

use anyhow::*;

use super::super::{AttachmentActor, AttachmentStatus};

pub async fn commit(actor: &mut AttachmentActor) -> Result<()> {
    match &mut actor.status {
        AttachmentStatus::Uninitialized => {
            unreachable!()
        }

        AttachmentStatus::Pending { file, uploaded_bytes } => {
            if *uploaded_bytes != actor.size {
                return Err(anyhow!("Tried to commit partially uploaded attachment"));
            }

            file.sync_all().await
                .unwrap();

            actor.status = AttachmentStatus::Ready {
                active_download_tokens: Arc::new(Default::default()),
            };

            Ok(())
        }

        AttachmentStatus::Ready { .. } => {
            Err(anyhow!("This attachment has been already committed"))
        }
    }
}