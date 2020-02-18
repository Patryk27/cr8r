use anyhow::*;

use lib_interop::connection::Connection;

use crate::modules::app::AppConfig;

pub struct AppContext {
    config: AppConfig,
    conn: Option<Connection>,
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

    pub async fn conn(&mut self) -> Result<Connection> {
        if self.conn.is_none() {
            let config = &self.config.controller;

            self.conn = Some(Connection::new(
                config.address.to_owned(),
                config.secret.to_owned(),
            ).await?);
        }

        Ok(self.conn.clone().unwrap())
    }
}