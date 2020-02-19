use anyhow::*;

use lib_interop::models::DRunnerId;

use crate::system::Runner;

use super::super::{RunnerStoreActor, RunnerStoreError};

pub fn find_one(actor: &RunnerStoreActor, id: DRunnerId) -> Result<Runner> {
    actor.runners
        .get(&id)
        .cloned()
        .ok_or_else(|| RunnerStoreError::RunnerNotFound { id }.into())
}