use anyhow::*;

use lib_interop::proto::services::{PSyncRunnerHeartbeatReply, PSyncRunnerHeartbeatRequest};

use crate::system::RunnerStore;

pub async fn sync_heartbeat(
    runner_store: &RunnerStore,
    request: PSyncRunnerHeartbeatRequest,
) -> Result<PSyncRunnerHeartbeatReply> {
    let id = request.id.into();

    runner_store
        .find_one(id).await?
        .sync_heartbeat().await;

    Ok(PSyncRunnerHeartbeatReply::default())
}