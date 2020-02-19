use anyhow::*;
use tonic::transport::Channel;

use crate::connection::Connection;
use crate::models::{DExperimentId, DRunnerId};
use crate::proto::services::*;
use crate::proto::services::assignments_client::AssignmentsClient as AssignmentsClientInner;
use crate::proto::services::p_prepare_assignment_reply::Assignment;

#[derive(Clone)]
pub struct AssignmentClient {
    inner: AssignmentsClientInner<Channel>,
}

impl AssignmentClient {
    crate fn new(conn: Connection) -> Self {
        Self {
            inner: AssignmentsClientInner::with_interceptor(
                conn.channel,
                conn.interceptor,
            ),
        }
    }

    pub async fn prepare(&mut self, runner_id: DRunnerId) -> Result<Option<DExperimentId>> {
        let reply = self.inner
            .prepare_assignment(PPrepareAssignmentRequest { runner_id: runner_id.into() }).await?
            .into_inner()
            .assignment;

        if let Some(Assignment::ExperimentId(experiment_id)) = reply {
            Ok(Some(experiment_id.into()))
        } else {
            Ok(None)
        }
    }
}