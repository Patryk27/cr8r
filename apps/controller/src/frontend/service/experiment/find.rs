use anyhow::*;

use lib_interop::proto::controller::{PFindExperimentsReply, PFindExperimentsRequest};

use crate::backend::System;

// @todo filtering should happen inside `system`, not here
pub async fn find_experiments(system: &System, request: PFindExperimentsRequest) -> Result<PFindExperimentsReply> {
    let mut experiments = Vec::new();

    for experiment in system.find_experiments().await {
        let experiment = experiment
            .get_model()
            .await;

        if request.id > 0 && experiment.id.as_num() != request.id {
            continue;
        }

        experiments.push(experiment);
    }

    experiments.sort_unstable_by(|a, b| {
        let a = a.id.as_num();
        let b = b.id.as_num();

        a.cmp(&b).reverse()
    });

    let experiments = experiments
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(PFindExperimentsReply { experiments })
}