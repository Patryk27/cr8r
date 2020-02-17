use anyhow::*;

use lib_interop::proto::services::{PFindExperimentsReply, PFindExperimentsRequest};

use crate::system::ExperimentStore;

// @todo filtering should happen inside the `Experiments` module, not here
pub async fn find_experiments(
    experiment_store: &ExperimentStore,
    request: PFindExperimentsRequest,
) -> Result<PFindExperimentsReply> {
    let mut experiments = Vec::new();

    for experiment in experiment_store.find_all().await {
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