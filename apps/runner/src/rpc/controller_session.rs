use anyhow::*;

use lib_interop::connection::ControllerConnection;
use lib_interop::proto::models::{PRunnerId, PRunnerName};
use lib_interop::proto::services::{PHowdyRequest, PRegisterRunnerRequest};

#[derive(Clone)]
pub struct ControllerSession {
    pub conn: ControllerConnection,
    pub runner_id: PRunnerId,
}

impl ControllerSession {
    pub async fn open(conn: ControllerConnection, runner_name: PRunnerName) -> Result<Self> {
        let version = conn
            .controller()
            .howdy(PHowdyRequest {})
            .await?
            .into_inner()
            .version;

        // @todo ensure we're compatible with controller

        let runner_id = conn
            .runners()
            .register_runner(PRegisterRunnerRequest { name: runner_name })
            .await?
            .into_inner()
            .id;

        Ok(Self { conn, runner_id })
    }
}
