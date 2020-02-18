use anyhow::*;
use tonic::transport::Channel;

use crate::connection::Connection;
use crate::proto::services::*;
use crate::proto::services::controller_client::ControllerClient as ControllerClientInner;

#[derive(Clone)]
pub struct ControllerClient {
    inner: ControllerClientInner<Channel>,
}

impl ControllerClient {
    crate fn new(conn: Connection) -> Self {
        Self {
            inner: ControllerClientInner::with_interceptor(
                conn.channel,
                conn.interceptor,
            ),
        }
    }

    pub async fn howdy(&mut self) -> Result<PHowdyReply> {
        let reply = self.inner
            .howdy(PHowdyRequest::default())
            .await?
            .into_inner();

        Ok(reply)
    }
}