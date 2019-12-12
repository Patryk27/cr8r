use crate::core::Ecosystem;

pub use self::{
    compiler::Compiler,
    error::{Error, Result},
    experiment::Experiment,
    experiment_watcher::ExperimentWatcher,
    runner::Runner,
    system::System,
};

mod compiler;
mod error;
mod experiment;
mod experiment_watcher;
mod runner;
mod system;

pub fn start(ecosystem: Ecosystem) -> System {
    let compiler = Compiler::new(ecosystem);

    System::spawn(compiler)
}

#[macro_export]
macro_rules! id {
    () => {
        uuid::Uuid::new_v4()
            .to_hyphenated()
            .to_string()
    }
}