use anyhow::*;

pub use self::{
    assignments::*,
    attachment::*,
    attachments::*,
    compiler::*,
    config::*,
    ecosystem::*,
    experiment::*,
    experiments::*,
    runner::*,
    runners::*,
};

mod assignments;
mod attachment;
mod attachments;
mod compiler;
mod config;
mod ecosystem;
mod experiment;
mod experiments;
mod runner;
mod runners;

pub struct System {
    pub assignments: Assignments,
    pub attachments: Attachments,
    pub experiments: Experiments,
    pub runners: Runners,
}

pub fn start(config: SystemConfig) -> Result<System> {
    let compiler = Compiler::new(config.ecosystem)
        .context("Could not initialize experiment compiler")?;

    let assignments = Assignments::new();
    let attachments = Attachments::new(config.attachments)?;
    let experiments = Experiments::new(compiler);
    let runners = Runners::new();

    Ok(System {
        assignments,
        attachments,
        experiments,
        runners,
    })
}
