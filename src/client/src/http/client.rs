use reqwest::{Client as ReqwestClient, header::{self, HeaderMap, HeaderValue}, RequestBuilder};

use crate::Result;

mod endpoints;

pub struct Client {
    controller_address: String,
    client: ReqwestClient,
}

impl Client {
    pub fn new(controller_address: String, controller_secret: String) -> Result<Self> {
        let headers = {
            let mut headers = HeaderMap::new();

            headers.insert(
                header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", controller_secret)).unwrap(),
            );

            headers
        };

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Ok(Self { controller_address, client })
    }

    fn get(&mut self, url: &str) -> RequestBuilder {
        self.client.get(&format!("{}{}", self.controller_address, url))
    }

    fn post(&mut self, url: &str) -> RequestBuilder {
        self.client.post(&format!("{}{}", self.controller_address, url))
    }
}