use core::result;

use anyhow::*;
use tokio::stream::Stream;
use tokio::sync::mpsc::unbounded_channel;
use tonic::Status;

use lib_interop::proto::services::{PDownloadAttachmentReply, PDownloadAttachmentRequest};

use crate::system::AttachmentStore;

pub async fn download_attachment(
    attachment_store: &AttachmentStore,
    request: PDownloadAttachmentRequest,
) -> Result<impl Stream<Item=result::Result<PDownloadAttachmentReply, Status>>> {
    let (tx, rx) = unbounded_channel();
    Ok(rx)
}
