use anyhow::*;

use crate::config::Ecosystem;

pub use self::{
    assignments::Assignments,
    attachment::Attachment,
    attachments::Attachments,
    compiler::Compiler,
    experiment::Experiment,
    experiments::Experiments,
    runner::Runner,
    runners::Runners,
};

mod assignments;
mod attachment;
mod attachments;
mod compiler;
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

pub fn start(ecosystem: Ecosystem) -> Result<System> {
    let compiler = Compiler::new(ecosystem)
        .context("Could not initialize experiment compiler")?;

    let assignments = Assignments::new();
    let attachments = Attachments::new();
    let experiments = Experiments::new(compiler);
    let runners = Runners::new();

    Ok(System {
        assignments,
        attachments,
        experiments,
        runners,
    })
}
