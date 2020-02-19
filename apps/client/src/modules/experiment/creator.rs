use std::collections::HashMap;

use anyhow::*;
use tokio::sync::mpsc::unbounded_channel;

use lib_core_channel::{URx, UTx};
use lib_interop::connection::Connection;
use lib_interop::models::{DAttachmentId, DExperimentId};

use crate::modules::attachment::AttachmentUploaderProgress;
use crate::modules::definition::DefinitionArg;

mod create_experiment;
mod upload_dependencies;
mod validate_dependencies;

pub struct ExperimentCreator {
    conn: Connection,
    progress: UTx<ExperimentCreatorProgress>,
    attachments: HashMap<String, DAttachmentId>,
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

impl ExperimentCreator {
    pub fn new(conn: Connection) -> (Self, URx<ExperimentCreatorProgress>) {
        let (progress, progress_rx) = unbounded_channel();

        (Self {
            conn,
            progress,
            attachments: Default::default(),
        }, progress_rx)
    }

    pub async fn create(mut self, def: DefinitionArg) -> Result<DExperimentId> {
        self.validate_dependencies(&def).await?;
        self.upload_dependencies(&def).await?;
        self.create_experiment(def).await
    }
}