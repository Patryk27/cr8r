use anyhow::*;

use lib_interop::models::{DDefinition, DExperimentId};
use lib_interop::models::definition::DDependencySourceDef;

use crate::system::Experiment;

use super::super::ExperimentStoreActor;

pub async fn launch(actor: &mut ExperimentStoreActor, definition: DDefinition) -> Result<DExperimentId> {
    let mut attachments = Vec::new();

    for dep in &definition.dependencies {
        if let DDependencySourceDef::Path { attachment_id } = &dep.source {
            let attachment = actor.attachment_store
                .find_one(*attachment_id)
                .await?;

            attachments.push(attachment);
        }
    }

    let id = actor.next_id.inc();
    let jobs = actor.compiler.compile(&definition);

    actor.experiments.insert(id, Experiment::new(
        id,
        attachments,
        jobs,
    ));

    actor.waiting_experiments.push_back(id);

    Ok(id)
}