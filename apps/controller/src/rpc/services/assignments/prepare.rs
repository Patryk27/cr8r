use anyhow::*;

use lib_interop::proto::services::{PPrepareAssignmentReply, PPrepareAssignmentRequest};
use lib_interop::proto::services::p_prepare_assignment_reply::Assignment;

use crate::system::System;

pub async fn prepare_assignment(system: &System, request: PPrepareAssignmentRequest) -> Result<PPrepareAssignmentReply> {
    let experiment_id = system
        .experiments
        .prepare_assignment(request.runner_id.into())
        .await?;

    Ok(PPrepareAssignmentReply {
        assignment: experiment_id.map(|experiment_id| {
            Assignment::ExperimentId(experiment_id.into())
        }),
    })
}