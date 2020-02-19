use anyhow::*;
use tonic::transport::Channel;

use crate::connection::Connection;
use crate::conv;
use crate::models::{DRunner, DRunnerId, DRunnerName};
use crate::proto::services::*;
use crate::proto::services::runners_client::RunnersClient as RunnersClientInner;

#[derive(Clone)]
pub struct RunnerClient {
    inner: RunnersClientInner<Channel>,
}

impl RunnerClient {
    crate fn new(conn: Connection) -> Self {
        Self {
            inner: RunnersClientInner::with_interceptor(
                conn.channel,
                conn.interceptor,
            ),
        }
    }

    pub async fn find_many(&mut self) -> Result<Vec<DRunner>> {
        let runners = self.inner
            .find_runners(PFindRunnersRequest::default()).await?
            .into_inner()
            .runners;

        Ok(conv!(runners as [_?]))
    }

    pub async fn register(&mut self, name: DRunnerName) -> Result<DRunnerId> {
        let reply = self.inner
            .register_runner(PRegisterRunnerRequest { name: name.into() }).await?
            .into_inner();

        Ok(reply.id.into())
    }
}