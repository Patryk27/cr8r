use reqwest::StatusCode;

use lib_client_protocol::GetControllerStatusResponse;
use lib_protocol_core::ControllerStatus;

use crate::{Connector, error::Error, Result};

impl Connector {
    pub fn controller_status(&self) -> Result<ControllerStatus> {
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