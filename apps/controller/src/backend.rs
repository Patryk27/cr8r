use crate::config::Ecosystem;

pub use self::{
    compiler::Compiler,
    error::Result,
    experiment::Experiment,
    runner::Runner,
    system::System,
};

mod compiler;
mod error;
mod experiment;
mod runner;
mod system;

pub fn start(ecosystem: Ecosystem) -> Result<System> {
    let compiler = Compiler::new(ecosystem).unwrap(); // @todo
    let system = System::new(compiler);

    Ok(system)
}
