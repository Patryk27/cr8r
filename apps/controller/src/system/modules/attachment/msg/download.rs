use std::sync::atomic::Ordering;

use anyhow::*;

use super::super::{AttachmentActor, AttachmentDownloadToken, AttachmentStatus};

pub fn download(actor: &mut AttachmentActor) -> Result<AttachmentDownloadToken> {
    match &actor.status {
        AttachmentStatus::Ready { active_download_tokens } => {
            active_download_tokens.fetch_add(1, Ordering::SeqCst);

            Ok(AttachmentDownloadToken::new(
                actor.path.clone(),
                active_download_tokens.clone(),
            ))
        }

        _ => {
            Err(anyhow!("This attachment has not been committed yet"))
        }
    }
}