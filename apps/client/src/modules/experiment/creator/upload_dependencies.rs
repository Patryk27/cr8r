use anyhow::*;

use lib_core_channel::SendTo;

use crate::modules::attachment::AttachmentUploader;
use crate::modules::definition::{DefinitionArg, DependencyArg, DependencySourceArg};

use super::{ExperimentCreator, ExperimentCreatorProgress::*};

impl ExperimentCreator {
    pub(super) async fn upload_dependencies(&mut self, def: &DefinitionArg) -> Result<()> {
        if !def.contains_path_dependencies() {
            return Ok(());
        }

        UploadingDependencies.send_to(&self.progress);

        for dep in &def.dependencies {
            self.upload_dependency(dep)
                .await
                .with_context(|| format!("Could not upload dependency: {}", dep.name))?;
        }

        DependenciesUploaded.send_to(&self.progress);

        Ok(())
    }

    async fn upload_dependency(&mut self, dep: &DependencyArg) -> Result<()> {
        use DependencySourceArg::*;

        if let Path(path) = &dep.source {
            let (mut uploader, progress) = AttachmentUploader::new(
                self.conn.attachments(),
            );

            UploadingDependency {
                name: dep.name.to_string(),
                progress,
            }.send_to(&self.progress);

            let id = uploader
                .upload_dir(path)
                .await?;

            self.attachments.insert(dep.name.to_string(), id);
        }

        Ok(())
    }
}