use crate::core::Ecosystem;

pub use self::{
    compiler::*,
    error::*,
    experiment::*,
    experiment_watcher::*,
    runner::*,
    system::*,
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