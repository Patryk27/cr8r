use std::future::Future;
use std::path::PathBuf;
use std::result;

use anyhow::*;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::sync::mpsc;
use tokio::task::{JoinError, spawn};

use lib_core_channel::{BRx, SendTo};
use lib_interop::proto::services::p_upload_attachment_request::{Chunk, PContent, PMetadata};
use lib_interop::proto::services::PUploadAttachmentRequest;
use lib_interop::proto::models::PAttachmentId;

use super::{AttachmentUploader, AttachmentUploaderProgress::*};

pub type JoinResult<T> = result::Result<T, JoinError>;

const CHUNK_SIZE: u64 = 64 * 1024;

impl<'c> AttachmentUploader<'c> {
    pub(super) async fn upload(&mut self, archive: PathBuf) -> Result<PAttachmentId> {
        let (stream, stream_task) = self.spawn_uploading_stream(archive);

        let reply = self.ctxt
            .client()
            .await?
            .upload_attachment(stream)
            .await?;

        // @todo we gotta check whether we shouldn't use `select!` here (in case that the stream fails, silently, and
        //       then `upload_attachment()` fails first because of the lost channel)
        stream_task.await??;

        AttachmentUploaded
            .send_to(&self.progress);

        Ok(reply.id)
    }

    fn spawn_uploading_stream(&self, archive: PathBuf) -> (BRx<PUploadAttachmentRequest>, impl Future<Output=JoinResult<Result<()>>>) {
        let progress = self.progress.clone();

        let (mut stream, stream_rx) = mpsc::channel(1);

        let stream_task = spawn(async move {
            let archive = File::open(archive)
                .await?;

            let total_bytes = archive
                .metadata()
                .await?
                .len();

            stream.send(PUploadAttachmentRequest {
                chunk: Some(Chunk::Metadata(PMetadata {
                    name: "foo".to_string(),
                    size: total_bytes,
                })),
            }).await?;

            AttachmentCompressed { total_bytes }
                .send_to(&progress);

            let mut archive = BufReader::new(archive);
            let mut chunk = [0u8; CHUNK_SIZE as usize];
            let mut sent_bytes = 0;

            loop {
                let chunk_size = archive
                    .read(&mut chunk)
                    .await?;

                if chunk_size == 0 {
                    break;
                }

                let chunk = {
                    let mut content = chunk.to_vec();

                    content.truncate(chunk_size);

                    Chunk::Content(PContent {
                        content,
                    })
                };

                stream
                    .send(PUploadAttachmentRequest { chunk: Some(chunk) })
                    .await?;

                UploadingAttachment { sent_bytes }
                    .send_to(&progress);

                sent_bytes += chunk_size as u64;
            }

            Ok(())
        });

        (stream_rx, stream_task)
    }
}