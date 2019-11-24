use reqwest::StatusCode;

use lib_protocol::ControllerStatus;
use lib_protocol::targets::client::GetControllerStatusResponse;

use crate::{Client, error::Error, Result};

impl Client {
    pub fn controller_status(&mut self) -> Result<ControllerStatus> {
        let mut response = self.get("/controller/status").send()?;

        match response.status() {
            StatusCode::OK => {
                Ok((response.json()?: GetControllerStatusResponse).status)
            }

            status => {
                Err(Error::FailedToProcessResponse {
                    source: format!("Server returned an unexpected status code `{}`", status).into(),
                })
            }
        }
    }
}