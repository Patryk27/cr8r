use anyhow::*;

use super::super::{AttachmentActor, AttachmentState};

pub async fn commit(actor: &mut AttachmentActor) -> Result<()> {
    match &mut actor.state {
        AttachmentState::Uninitialized => {
            unreachable!()
        }

        AttachmentState::Pending { file, uploaded_bytes } => {
            if *uploaded_bytes != actor.size {
                return Err(anyhow!("Tried to commit partially uploaded attachment"));
            }

            file.sync_all()
                .await
                .unwrap();

            actor.state = AttachmentState::Ready;

            Ok(())
        }

        AttachmentState::Ready => {
            Err(anyhow!("This attachment has been already committed"))
        }
    }
}