use serde::{Deserialize, Serialize};

use crate::{RunnerId, RunnerName, RunnerStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct Runner {
    pub id: RunnerId,
    pub name: RunnerName,
    pub status: RunnerStatus,
}