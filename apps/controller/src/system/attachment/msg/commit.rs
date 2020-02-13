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

            file.sync_all()
                .await
                .unwrap();

            actor.status = AttachmentStatus::Ready;

            Ok(())
        }

        AttachmentStatus::Ready => {
            Err(anyhow!("This attachment has been already committed"))
        }
    }
}