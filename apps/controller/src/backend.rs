use anyhow::*;

use crate::config::Ecosystem;

pub use self::{
    compiler::Compiler,
    experiment::Experiment,
    runner::Runner,
    system::System,
};

mod compiler;
mod experiment;
mod runner;
mod system;

pub fn start(ecosystem: Ecosystem) -> Result<System> {
    let compiler = Compiler::new(ecosystem)
        .context("Could not initialize experiment compiler")?;

    let system = System::new(compiler);

    Ok(system)
}
