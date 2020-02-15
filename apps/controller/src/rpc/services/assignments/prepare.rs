use anyhow::*;

use lib_interop::proto::services::{PPrepareAssignmentReply, PPrepareAssignmentRequest};
use lib_interop::proto::services::p_prepare_assignment_reply::Assignment;

use crate::system::Experiments;

pub async fn prepare_assignment(
    experiments: &Experiments,
    request: PPrepareAssignmentRequest,
) -> Result<PPrepareAssignmentReply> {
    let runner_id = request.runner_id.into();

    let experiment_id = experiments
        .prepare_assignment(runner_id)
        .await?;

    Ok(PPrepareAssignmentReply {
        assignment: experiment_id.map(|experiment_id| {
            Assignment::ExperimentId(experiment_id.into())
        }),
    })
}