use std::io::BufWriter;
use std::path::PathBuf;

use anyhow::*;
use tar::Builder as TarBuilder;
use tokio::task::spawn_blocking;

use lib_core_channel::SendTo;
use lib_core_tempfile::TempFile;

use super::{AttachmentUploader, AttachmentUploaderProgress::*};

impl AttachmentUploader {
    pub(super) async fn compress_dir(&mut self, path: PathBuf) -> Result<TempFile> {
        CompressingAttachment.send_to(&self.progress);

        let archive = TempFile::new()
            .await?;

        spawn_blocking(move || {
            let tar_buffer = BufWriter::new(
                archive.std_file()
            );

            let mut tar = TarBuilder::new(tar_buffer);

            // @todo we shouldn't upload `.git` and `target`

            tar.append_dir_all(".", path)?;
            tar.finish()?;

            drop(tar);

            Ok(archive)
        }).await?
    }
}