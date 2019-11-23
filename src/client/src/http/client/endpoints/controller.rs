use reqwest::StatusCode;

use lib_protocol as proto;

use crate::{Client, error::Error, Result};

impl Client {
    pub fn controller_status(&mut self) -> Result<proto::ControllerStatus> {
        let mut response = self.get("/controller/status").send()?;

        match response.status() {
            StatusCode::OK => {
                let response: proto::client::GetControllerStatusResponse = response.json()?;
                Ok(response.status)
            }

            status => {
                Err(Error::FailedToProcessResponse {
                    source: format!("Server returned an unexpected status code `{}`", status).into(),
                })
            }
        }
    }
}