use structopt::StructOpt;

use crate::{Result, System};

pub use self::{
    controller::*,
    experiment::*,
};

mod controller;
mod experiment;

#[derive(Debug, StructOpt)]
pub enum Command {
    Controller(ControllerCommand),
    Experiment(ExperimentCommand),
}

impl Command {
    pub fn run(self, system: System) -> Result<()> {
        match self {
            Command::Controller(cmd) => cmd.run(system),
            Command::Experiment(cmd) => cmd.run(system),
        }
    }
}