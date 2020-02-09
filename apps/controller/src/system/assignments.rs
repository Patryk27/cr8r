use anyhow::*;

use lib_interop::domain::{DAssignment, DRunnerId};

#[derive(Clone)]
pub struct Assignments {
    //
}

impl Assignments {
    pub fn new() -> Self {
        Self {
            //
        }
    }

    pub async fn prepare(&self, runner_id: DRunnerId) -> Result<Option<DAssignment>> {
        unimplemented!()
    }
}