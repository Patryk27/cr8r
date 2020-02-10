use std::io as std_io;
use std::path::PathBuf;

use anyhow::*;
use tokio::fs as tokio_fs;
use tokio::io as tokio_io;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use tokio::task;

use lib_core_channel::{SendTo, URx, UTx};
use lib_core_tempfile::TempFile;
use lib_interop::proto::controller::p_upload_attachment_request::{Chunk, PContent, PMetadata};
use lib_interop::proto::controller::PUploadAttachmentRequest;
use lib_interop::proto::core::PAttachmentId;

use crate::modules::app::AppContext;

const CHUNK_SIZE: u64 = 16 * 1024;

pub struct AttachmentUploader<'c> {
    ctxt: &'c mut AppContext,
    tx: UTx<AttachmentUploaderProgress>,
}

pub enum AttachmentUploaderProgress {
    CompressingAttachment,

    AttachmentCompressed {
        total_bytes: u64,
    },

    UploadingAttachment {
        sent_bytes: u64,
    },

    AttachmentUploaded,
}

impl<'c> AttachmentUploader<'c> {
    pub fn new(ctxt: &'c mut AppContext) -> (Self, URx<AttachmentUploaderProgress>) {
        let (tx, rx) = mpsc::unbounded_channel();

        (Self { ctxt, tx }, rx)
    }

    pub async fn upload_dir(&mut self, dir: impl Into<PathBuf>) -> Result<PAttachmentId> {
        let dir = dir.into();

        let archive = self
            .compress(dir)
            .await?;

        self.upload(archive.path_buf())
            .await
    }

    async fn compress(&mut self, dir: PathBuf) -> Result<TempFile> {
        AttachmentUploaderProgress::CompressingAttachment
            .send_to(&self.tx);

        let archive = TempFile::new()
            .await?;

        task::spawn_blocking(move || {
            let mut tar = tar::Builder::new(
                std_io::BufWriter::new(
                    archive.file(),
                )
            );

            tar.append_dir_all(".", dir)?;
            tar.finish()?;

            drop(tar);

            Ok(archive)
        }).await.unwrap()
    }

    async fn upload(&mut self, archive: PathBuf) -> Result<PAttachmentId> {
        let tx = self.tx.clone();
        let (mut rpc_tx, rpc_rx) = mpsc::channel(1);

        task::spawn(async move {
            let archive = tokio_fs::File::open(archive)
                .await
                .unwrap();

            let archive_size = archive.metadata()
                .await
                .unwrap()
                .len();

            rpc_tx
                .send(PUploadAttachmentRequest {
                    chunk: Some(Chunk::Metadata(PMetadata {
                        name: "foo".to_string(),
                        size: archive_size,
                    })),
                })
                .await
                .unwrap();

            AttachmentUploaderProgress::AttachmentCompressed {
                total_bytes: archive_size,
            }.send_to(&tx);

            let mut archive = tokio_io::BufReader::new(archive);
            let mut chunk = [0u8; CHUNK_SIZE as usize];
            let mut sent_bytes = 0;

            loop {
                let chunk_size = archive
                    .read(&mut chunk)
                    .await
                    .unwrap();

                if chunk_size == 0 {
                    break;
                }

                rpc_tx.send(PUploadAttachmentRequest {
                    chunk: Some(Chunk::Content(PContent {
                        content: chunk.to_vec(),
                        size: chunk_size as u64,
                    }))
                }).await.unwrap();

                AttachmentUploaderProgress::UploadingAttachment { sent_bytes }
                    .send_to(&tx);

                sent_bytes += chunk_size as u64;
            }
        });

        let reply = self.ctxt
            .client()
            .await?
            .upload_attachment(rpc_rx)
            .await?;

        AttachmentUploaderProgress::AttachmentUploaded
            .send_to(&self.tx);

        Ok(reply.id)
    }
}
