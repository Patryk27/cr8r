use core::result;

use anyhow::*;
use tokio::stream::Stream;
use tokio::sync::mpsc::unbounded_channel;
use tonic::Status;

use lib_interop::proto::services::{PDownloadAttachmentReply, PDownloadAttachmentRequest};

use crate::system::Attachments;

pub async fn download_attachment(
    attachments: &Attachments,
    request: PDownloadAttachmentRequest,
) -> Result<impl Stream<Item=result::Result<PDownloadAttachmentReply, Status>>> {
    let (tx, rx) = unbounded_channel();
    Ok(rx)
}
