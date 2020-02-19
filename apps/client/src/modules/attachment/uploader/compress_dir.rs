use std::fs::read_dir;
use std::io::BufWriter;
use std::path::PathBuf;

use anyhow::*;
use tar::Builder as TarBuilder;
use tokio::task::spawn_blocking;

use lib_core_channel::SendTo;
use lib_core_tempfile::TempFile;

use super::{AttachmentUploader, AttachmentUploaderProgress::*};

const SKIPPABLE_DIRS: &[&str] = &[
    ".git",
    "target",
];

impl AttachmentUploader {
    pub(super) async fn compress_dir(&mut self, path: PathBuf) -> Result<TempFile> {
        CompressingAttachment.send_to(&self.progress);

        let archive = TempFile::new().await?;

        spawn_blocking(move || {
            ensure!(path.is_dir(), "Given path is not a directory");
            ensure!(path.join("Cargo.toml").exists(), "Given directory does not contain `Cargo.toml`");

            let tar_buffer = BufWriter::new(
                archive.std_file(),
            );

            let mut tar = TarBuilder::new(tar_buffer);

            for entry in read_dir(&path)? {
                let entry = entry?;
                let entry_path = entry.path();

                if let Some(entry_name) = entry_path.file_name() {
                    if SKIPPABLE_DIRS.iter().any(|dir| dir == &entry_name) {
                        continue;
                    }

                    if entry_path.is_dir() {
                        tar.append_dir_all(entry_name, &entry_path)?;
                    } else {
                        tar.append_path_with_name(&entry_path, entry_name)?;
                    }
                }
            }

            tar.into_inner()?;

            Ok(archive)
        }).await?
    }
}