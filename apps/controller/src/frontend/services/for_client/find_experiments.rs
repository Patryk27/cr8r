use lib_interop::protocol::for_client::{PFindExperimentsReply, PFindExperimentsRequest};

use crate::backend::System;

// @todo filtering should happen inside `system`, not here
pub async fn find_experiments(system: &System, request: PFindExperimentsRequest) -> PFindExperimentsReply {
    let mut experiments = Vec::new();

    for experiment in system.find_experiments().await {
        let experiment = experiment
            .get_model()
            .await;

        let mut matches = true;

        if !request.id.is_empty() {
            matches = experiment.id.as_str() == &request.id;
        }

        if matches {
            experiments.push(experiment.into());
        }
    }

    PFindExperimentsReply { experiments }
}