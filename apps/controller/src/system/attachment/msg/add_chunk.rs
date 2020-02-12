use anyhow::*;
use log::*;
use tokio::io::AsyncWriteExt;

use lib_interop::proto::core::PAttachmentSize;

use super::super::{AttachmentActor, AttachmentState};

pub async fn add_chunk(actor: &mut AttachmentActor, chunk: Vec<u8>) -> Result<()> {
    match &mut actor.state {
        AttachmentState::Uninitialized => {
            unreachable!()
        }

        AttachmentState::Pending { file, uploaded_bytes } => {
            let chunk_len = chunk.len() as PAttachmentSize;

            if *uploaded_bytes + chunk_len > actor.size {
                return Err(anyhow!("Tried to upload more bytes than originally reported"));
            }

            match file.write(&chunk).await {
                Ok(_) => {
                    *uploaded_bytes += chunk_len;

                    Ok(())
                }

                Err(err) => {
                    // @todo moar logs
                    error!("Could not write attachment's chunk to the filesystem: {:?}", err);

                    Err(anyhow!("Could not write attachment's chunk to the filesystem: {:?}", err))
                }
            }
        }

        AttachmentState::Ready => {
            Err(anyhow!("This attachment has been already committed"))
        }
    }
}

