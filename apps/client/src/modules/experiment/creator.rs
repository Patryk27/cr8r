use anyhow::*;
use tokio::fs;
use tokio::sync::mpsc::unbounded_channel;

use lib_core_channel::{SendTo, URx, UTx};
use lib_interop::proto::core::PExperimentId;

use crate::modules::app::AppContext;
use crate::modules::attachment::{AttachmentUploader, AttachmentUploaderProgress};
use crate::modules::definition::{DefinitionArg, DependencySourceArg};

pub struct ExperimentCreator<'c> {
    ctxt: &'c mut AppContext,
    tx: UTx<ExperimentCreatorProgress>,
}

pub enum ExperimentCreatorProgress {
    ValidatingDependencies,

    ValidatingDependency {
        name: String,
    },

    UploadingDependencies,

    UploadingDependency {
        name: String,
        progress: URx<AttachmentUploaderProgress>,
    },

    CreatingExperiment,

    ExperimentCreated,
}

impl<'c> ExperimentCreator<'c> {
    pub fn new(ctxt: &'c mut AppContext) -> (Self, URx<ExperimentCreatorProgress>) {
        let (tx, rx) = unbounded_channel();

        (Self { ctxt, tx }, rx)
    }

    pub async fn create(mut self, def: DefinitionArg) -> Result<PExperimentId> {
        self.validate_dependencies(&def)
            .await?;

        self.upload_dependencies(&def)
            .await?;

        self.create_experiment(def)
            .await
    }

    async fn validate_dependencies(&mut self, def: &DefinitionArg) -> Result<()> {
        ExperimentCreatorProgress::ValidatingDependencies
            .send_to(&self.tx);

        for dep in &def.dependencies {
            ExperimentCreatorProgress::ValidatingDependency {
                name: dep.name.to_string(),
            }.send_to(&self.tx);

            if let DependencySourceArg::Path(path) = &dep.source {
                if fs::metadata(path).await.is_err() {
                    return Err(anyhow!(
                        "Dependency `{}` refers to a non-existing path `{}`",
                        dep.name, path,
                    ));
                }
            }
        }

        Ok(())
    }

    async fn upload_dependencies(&mut self, def: &DefinitionArg) -> Result<()> {
        if !def.contains_path_deps() {
            return Ok(());
        }

        ExperimentCreatorProgress::UploadingDependencies
            .send_to(&self.tx);

        for dep in &def.dependencies {
            if let DependencySourceArg::Path(path) = &dep.source {
                let (mut uploader, progress) = AttachmentUploader::new(self.ctxt);

                ExperimentCreatorProgress::UploadingDependency {
                    name: dep.name.to_string(),
                    progress,
                }.send_to(&self.tx);

                uploader
                    .upload_dir(path)
                    .await
                    .with_context(|| format!("Could not upload `{}`", dep.name))?;
            }
        }

        Ok(())
    }

    async fn create_experiment(&mut self, def: DefinitionArg) -> Result<PExperimentId> {
        ExperimentCreatorProgress::CreatingExperiment
            .send_to(&self.tx);

        unimplemented!()
    }
}