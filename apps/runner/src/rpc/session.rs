use anyhow::*;

use lib_interop::connection::Connection;
use lib_interop::models::{DRunnerId, DRunnerName};

#[derive(Clone)]
pub struct Session {
    conn: Connection,
    runner_id: DRunnerId,
    runner_name: DRunnerName,
}

impl Session {
    pub async fn new(conn: Connection, runner_name: DRunnerName) -> Result<Self> {
        let version = conn
            .controller()
            .howdy().await?
            .version;

        // @todo ensure we're compatible with controller

        let runner_id = conn
            .runners()
            .register(runner_name.clone()).await?;

        Ok(Self {
            conn,
            runner_id,
            runner_name,
        })
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    pub fn runner_id(&self) -> DRunnerId {
        self.runner_id
    }

    pub fn runner_name(&self) -> &DRunnerName {
        &self.runner_name
    }
}
