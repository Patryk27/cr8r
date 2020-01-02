use std::str::FromStr;

use hyper::header::HeaderValue;
use hyper::Uri;
use tonic::transport::Channel;

use lib_interop::protocol::for_client::for_client_client::ForClientClient;

use crate::{Config, Result};

pub struct System {
    config: Config,
    client: Option<ForClientClient<Channel>>,
}

impl System {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: None,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub async fn client(&mut self) -> Result<&mut ForClientClient<Channel>> {
        if self.client.is_none() {
            self.client = Some(self.connect().await?);
        }

        Ok(self.client
            .as_mut()
            .unwrap())
    }

    async fn connect(&self) -> Result<ForClientClient<Channel>> {
        let auth = self.config.controller.secret
            .as_ref()
            .map(|secret| format!("Bearer {}", secret))
            .map(|secret| HeaderValue::from_str(&secret))
            .transpose()?;

        let uri = Uri::from_str(&self.config.controller.address)?;

        let channel = Channel::builder(uri)
            .intercept_headers(move |headers| {
                if let Some(auth) = &auth {
                    headers.insert("authorization", auth.to_owned());
                }
            })
            .connect()
            .await?;

        Ok(ForClientClient::new(channel))
    }
}