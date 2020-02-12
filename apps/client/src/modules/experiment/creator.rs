use std::collections::HashMap;

use anyhow::*;
use tokio::sync::mpsc::unbounded_channel;

use lib_core_channel::{URx, UTx};
use lib_interop::domain::DExperimentId;
use lib_interop::proto::core::PAttachmentId;

use crate::modules::app::AppContext;
use crate::modules::attachment::AttachmentUploaderProgress;
use crate::modules::definition::DefinitionArg;

mod create_experiment;
mod upload_dependencies;
mod validate_dependencies;

pub struct ExperimentCreator<'c> {
    ctxt: &'c mut AppContext,
    progress: UTx<ExperimentCreatorProgress>,
    attachments: HashMap<String, PAttachmentId>,
}

pub enum ExperimentCreatorProgress {
    ValidatingDependencies,

    ValidatingDependency {
        name: String,
    },

    DependenciesValidated,

    UploadingDependencies,

    UploadingDependency {
        name: String,
        progress: URx<AttachmentUploaderProgress>,
    },

    DependenciesUploaded,

    CreatingExperiment,

    ExperimentCreated {
        id: DExperimentId,
    },
}

impl<'c> ExperimentCreator<'c> {
    pub fn new(ctxt: &'c mut AppContext) -> (Self, URx<ExperimentCreatorProgress>) {
        let (progress, progress_rx) = unbounded_channel();

        (Self {
            ctxt,
            progress,
            attachments: Default::default(),
        }, progress_rx)
    }

    pub async fn create(mut self, def: DefinitionArg) -> Result<DExperimentId> {
        self.validate_dependencies(&def)
            .await?;

        self.upload_dependencies(&def)
            .await?;

        self.create_experiment(def)
            .await
    }
}