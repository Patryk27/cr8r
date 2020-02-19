use std::future::Future;
use std::path::PathBuf;
use std::result;

use anyhow::*;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::stream::Stream;
use tokio::sync::mpsc::channel;
use tokio::task::{JoinError, spawn};

use lib_core_channel::SendTo;
use lib_interop::models::DAttachmentId;
use lib_interop::proto::services::p_upload_attachment_request::{Chunk, PBody, PMetadata};

use super::{AttachmentUploader, AttachmentUploaderProgress::*};

pub type JoinResult<T> = result::Result<T, JoinError>;

const CHUNK_SIZE: usize = 64 * 1024;

impl AttachmentUploader {
    pub(super) async fn upload_archive(&mut self, archive: PathBuf) -> Result<DAttachmentId> {
        let (stream, stream_task) = self.spawn_uploading_stream(archive);

        let id = self.client
            .upload(stream).await?;

        // @todo we gotta check whether we shouldn't use `select!` here (in case that the stream fails, silently, and
        //       then `upload_attachment()` fails first because of the lost channel)
        stream_task.await??;

        AttachmentUploaded
            .send_to(&self.progress);

        Ok(id)
    }

    fn spawn_uploading_stream(&self, archive: PathBuf) -> (
        impl Stream<Item=Chunk>,
        impl Future<Output=JoinResult<Result<()>>>,
    ) {
        let progress = self.progress.clone();

        let (mut stream, stream_rx) = channel(1);

        let stream_task = spawn(async move {
            let archive = File::open(archive).await?;

            let total_bytes = archive
                .metadata().await?
                .len();

            stream.send(Chunk::Metadata(PMetadata {
                name: "foo".to_string(),
                size: total_bytes,
            })).await?;

            AttachmentCompressed { total_bytes }
                .send_to(&progress);

            let mut archive = BufReader::new(archive);
            let mut chunk = [0u8; CHUNK_SIZE];
            let mut sent_bytes = 0;

            loop {
                let read_bytes = archive.read(&mut chunk).await?;

                if read_bytes == 0 {
                    break;
                }

                let chunk = {
                    let mut body = chunk.to_vec();

                    body.truncate(read_bytes);

                    Chunk::Body(PBody { body })
                };

                stream.send(chunk).await?;

                UploadingAttachment { sent_bytes }
                    .send_to(&progress);

                sent_bytes += read_bytes as u64;
            }

            Ok(())
        });

        (stream_rx, stream_task)
    }
}