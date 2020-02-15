use anyhow::*;
use tonic::transport::Channel;

use lib_interop::connection::ControllerConnection;
use lib_interop::proto::services::{
    assignments_client::AssignmentsClient,
    attachments_client::AttachmentsClient,
    controller_client::ControllerClient,
    events_client::EventsClient,
    experiments_client::ExperimentsClient,
    jobs_client::JobsClient,
    reports_client::ReportsClient,
    runners_client::RunnersClient,
};

use crate::modules::app::AppConfig;

pub struct AppContext {
    config: AppConfig,
    conn: Option<ControllerConnection>,
}

impl AppContext {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            conn: None,
        }
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub async fn assignments(&mut self) -> Result<AssignmentsClient<Channel>> {
        self.conn()
            .await
            .map(|conn| conn.assignments())
    }

    pub async fn attachments(&mut self) -> Result<AttachmentsClient<Channel>> {
        self.conn()
            .await
            .map(|conn| conn.attachments())
    }

    pub async fn controller(&mut self) -> Result<ControllerClient<Channel>> {
        self.conn()
            .await
            .map(|conn| conn.controller())
    }

    pub async fn events(&mut self) -> Result<EventsClient<Channel>> {
        self.conn()
            .await
            .map(|conn| conn.events())
    }

    pub async fn experiments(&mut self) -> Result<ExperimentsClient<Channel>> {
        self.conn()
            .await
            .map(|conn| conn.experiments())
    }

    pub async fn jobs(&mut self) -> Result<JobsClient<Channel>> {
        self.conn()
            .await
            .map(|conn| conn.jobs())
    }

    pub async fn reports(&mut self) -> Result<ReportsClient<Channel>> {
        self.conn()
            .await
            .map(|conn| conn.reports())
    }

    pub async fn runners(&mut self) -> Result<RunnersClient<Channel>> {
        self.conn()
            .await
            .map(|conn| conn.runners())
    }

    async fn conn(&mut self) -> Result<&ControllerConnection> {
        if self.conn.is_none() {
            let config = &self.config.controller;

            self.conn = Some(ControllerConnection::new(
                config.address.to_owned(),
                config.secret.to_owned(),
            ).await?);
        }

        Ok(self.conn.as_mut().unwrap())
    }
}