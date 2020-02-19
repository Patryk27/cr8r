use thiserror::Error;

use lib_interop::models::DRunnerId;

#[derive(Debug, Error)]
pub enum RunnerStoreError {
    #[error("Runner [id={id}] could not be found")]
    RunnerNotFound {
        id: DRunnerId,
    }
}