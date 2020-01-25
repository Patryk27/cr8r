use anyhow::*;

use lib_interop::proto::controller::{PRegisterRunnerReply, PRegisterRunnerRequest};

use crate::backend::System;

pub async fn register_runner(system: &System, request: PRegisterRunnerRequest) -> Result<PRegisterRunnerReply> {
    let id = system
        .create_runner(request.name.into())
        .await?;

    Ok(PRegisterRunnerReply {
        id: id.into(),
    })
}