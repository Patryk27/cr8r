use anyhow::*;

use lib_interop::proto::services::{PRegisterRunnerReply, PRegisterRunnerRequest};

use crate::system::RunnerStore;

pub async fn register_runner(
    runner_store: &RunnerStore,
    request: PRegisterRunnerRequest,
) -> Result<PRegisterRunnerReply> {
    let name = request.name.into();

    let id = runner_store
        .register(name)
        .await?;

    Ok(PRegisterRunnerReply {
        id: id.into(),
    })
}