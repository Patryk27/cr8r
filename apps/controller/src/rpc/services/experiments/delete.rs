use anyhow::*;

use lib_interop::proto::services::{PDeleteExperimentReply, PDeleteExperimentRequest};

use crate::system::System;

pub async fn delete_experiment(system: &System, request: PDeleteExperimentRequest) -> Result<PDeleteExperimentReply> {
    let id = request.id.into();

    system.experiments
        .delete(id)
        .await?;

    Ok(PDeleteExperimentReply {})
}