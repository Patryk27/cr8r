use serde::{Deserialize, Serialize};

use lib_protocol_core::ControllerStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetControllerStatusRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetControllerStatusResponse {
    pub status: ControllerStatus,
}