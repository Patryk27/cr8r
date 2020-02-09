use anyhow::*;

use lib_interop::proto::controller::{PRegisterRunnerReply, PRegisterRunnerRequest};

use crate::system::System;

pub async fn register_runner(system: &System, request: PRegisterRunnerRequest) -> Result<PRegisterRunnerReply> {
    let name = request.name.into();

    let id = system
        .runners
        .register(name)
        .await?;

    Ok(PRegisterRunnerReply {
        id: id.into(),
    })
}