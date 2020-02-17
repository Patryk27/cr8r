use anyhow::*;

pub use self::{
    attachment::*,
    attachment_store::*,
    compiler::*,
    config::*,
    ecosystem::*,
    experiment::*,
    experiment_store::*,
    runner::*,
    runner_store::*,
};

mod attachment;
mod attachment_store;
mod compiler;
mod config;
mod ecosystem;
mod experiment;
mod experiment_store;
mod runner;
mod runner_store;

pub struct System {
    pub attachment_store: AttachmentStore,
    pub experiment_store: ExperimentStore,
    pub runner_store: RunnerStore,
}

pub async fn start(config: SystemConfig) -> Result<System> {
    let compiler = Compiler::new(config.ecosystem)
        .context("Could not initialize experiment compiler")?;

    let attachment_store = AttachmentStore::new(config.attachments)
        .await
        .context("Could not initialize attachment store")?;

    let experiment_store = ExperimentStore::new(attachment_store.clone(), compiler);
    let runner_store = RunnerStore::new();

    Ok(System {
        attachment_store,
        experiment_store,
        runner_store,
    })
}
