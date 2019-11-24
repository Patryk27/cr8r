use serde::{Deserialize, Serialize};

use crate::ControllerStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetControllerStatusRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetControllerStatusResponse {
    pub status: ControllerStatus,
}