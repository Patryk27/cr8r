use anyhow::*;

use lib_interop::proto::controller::{PGetAssignmentReply, PGetAssignmentRequest};

use crate::backend::System;

pub async fn get_assignment(system: &System, request: PGetAssignmentRequest) -> Result<PGetAssignmentReply> {
    let assignment = system
        .get_assignment(request.runner_id.into())
        .await?;

    Ok(PGetAssignmentReply {
        assignment: assignment.map(Into::into),
    })
}