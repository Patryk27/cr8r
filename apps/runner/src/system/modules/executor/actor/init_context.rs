use std::collections::{HashMap, VecDeque};

use anyhow::*;
use log::*;

use lib_interop::models::{DAttachmentId, DJob};

use crate::system::Attachment;

use super::{ExecutorActor, ExecutorContext};

impl ExecutorActor {
    pub(super) async fn init_context(&mut self) -> Result<ExecutorContext> {
        let attachments = self
            .fetch_attachments().await
            .context("Could not fetch experiment's attachments")?;

        let jobs = self
            .fetch_jobs().await
            .context("Could not fetch experiment's jobs")?;

        Ok(ExecutorContext {
            attachments,
            jobs,
        })
    }

    async fn fetch_attachments(&mut self) -> Result<HashMap<DAttachmentId, Attachment>> {
        debug!("Fetching experiment's attachments");

        let experiment_attachments = self.session
            .conn()
            .attachments()
            .find_many(self.experiment_id).await?;

        let mut attachments = HashMap::with_capacity(
            experiment_attachments.len(),
        );

        for experiment_attachment in experiment_attachments {
            let attachment = self.attachment_store.download(experiment_attachment.id).await?;

            attachments.insert(experiment_attachment.id, attachment);
        }

        Ok(attachments)
    }

    async fn fetch_jobs(&mut self) -> Result<VecDeque<DJob>> {
        debug!("Fetching experiment's job");

        let jobs = self.session
            .conn()
            .jobs()
            .find_many(self.experiment_id).await?;

        Ok(jobs
            .into_iter()
            .collect())
    }
}