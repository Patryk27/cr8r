use anyhow::*;

use lib_interop::proto::services::{PDeleteExperimentReply, PDeleteExperimentRequest};

use crate::system::Experiments;

pub async fn delete_experiment(
    experiments: &Experiments,
    request: PDeleteExperimentRequest,
) -> Result<PDeleteExperimentReply> {
    let id = request.id.into();

    experiments
        .delete(id)
        .await?;

    Ok(PDeleteExperimentReply {})
}