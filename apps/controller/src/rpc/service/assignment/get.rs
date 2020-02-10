use anyhow::*;

use lib_interop::proto::controller::{PGetAssignmentReply, PGetAssignmentRequest};

use crate::system::System;

// @todo rename me maybe?
pub async fn get_assignment(system: &System, request: PGetAssignmentRequest) -> Result<PGetAssignmentReply> {
    let assignment = system
        .assignments
        .prepare(request.runner_id.into())
        .await?;

    Ok(PGetAssignmentReply {
        assignment: assignment.map(Into::into),
    })
}