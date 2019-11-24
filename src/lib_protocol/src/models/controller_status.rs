use serde::{Deserialize, Serialize};

use crate::{Experiment, Runner};

#[derive(Debug, Serialize, Deserialize)]
pub struct ControllerStatus {
    pub experiments: Vec<Experiment>,
    pub runners: Vec<Runner>,
}