use anyhow::*;

use lib_interop::proto::services::{PFindExperimentsReply, PFindExperimentsRequest};

use crate::system::Experiments;

// @todo filtering should happen inside the `Experiments` module, not here
pub async fn find_experiments(
    experiments: &Experiments,
    request: PFindExperimentsRequest,
) -> Result<PFindExperimentsReply> {
    let mut found_experiments = Vec::new();

    for experiment in experiments.find_all().await {
        let experiment = experiment
            .get_model()
            .await;

        if request.id > 0 && experiment.id.as_num() != request.id {
            continue;
        }

        found_experiments.push(experiment);
    }

    found_experiments.sort_unstable_by(|a, b| {
        let a = a.id.as_num();
        let b = b.id.as_num();

        a.cmp(&b).reverse()
    });

    let experiments = found_experiments
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(PFindExperimentsReply { experiments })
}