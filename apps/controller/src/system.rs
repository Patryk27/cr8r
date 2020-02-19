use anyhow::*;

pub use self::{
    config::*,
    event::*,
    event_bus::*,
    modules::*,
};

mod config;
mod event;
mod event_bus;
mod modules;

pub struct System {
    pub attachment_store: AttachmentStore,
    pub experiment_store: ExperimentStore,
    pub runner_store: RunnerStore,
}

pub async fn start(config: SystemConfig) -> Result<System> {
    let compiler = Compiler::new(config.ecosystem)
        .context("Could not initialize experiment compiler")?;

    let bus = SystemEventBus::new();

    Logger::new(
        bus.clone(),
    );

    let attachment_store = AttachmentStore::new(
        bus.clone(),
        config.attachments,
    ).await.context("Could not initialize attachment store")?;

    let experiment_store = ExperimentStore::new(
        bus.clone(),
        attachment_store.clone(),
        compiler,
    );

    let runner_store = RunnerStore::new(
        bus.clone(),
    );

    Ok(System {
        attachment_store,
        experiment_store,
        runner_store,
    })
}
