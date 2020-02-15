use anyhow::*;

use lib_interop::proto::services::{PRegisterRunnerReply, PRegisterRunnerRequest};

use crate::system::Runners;

pub async fn register_runner(
    runners: &Runners,
    request: PRegisterRunnerRequest,
) -> Result<PRegisterRunnerReply> {
    let name = request.name.into();

    let id = runners
        .register(name)
        .await?;

    Ok(PRegisterRunnerReply {
        id: id.into(),
    })
}