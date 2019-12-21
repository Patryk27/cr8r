use lib_interop::protocol::for_runner::{PRegisterReply, PRegisterRequest};

use crate::backend::{Result, System};

pub async fn register(system: &System, request: PRegisterRequest) -> Result<PRegisterReply> {
    let id = system
        .create_runner(request.name.into())
        .await?;

    Ok(PRegisterReply {
        id: id.into(),
    })
}