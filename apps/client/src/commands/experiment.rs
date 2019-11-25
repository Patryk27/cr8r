use structopt::StructOpt;

use crate::{Result, System};

pub use self::launch::*;

mod abort;
mod launch;
mod report;
mod status;
mod watch;

#[derive(Debug, StructOpt)]
pub enum ExperimentCommand {
    Abort {
        id: String,
    },

    Launch(LaunchExperimentCommand),

    Report {
        id: String,
    },

    Status {
        id: String,
    },

    Watch {
        id: String,
    },
}

impl ExperimentCommand {
    pub fn run(self, system: System) -> Result<()> {
        match self {
            ExperimentCommand::Abort { id: experiment_id } => abort::run(system, experiment_id),
            ExperimentCommand::Report { id: experiment_id } => report::run(system, experiment_id),
            ExperimentCommand::Launch(cmd) => cmd.run(system),
            ExperimentCommand::Status { id: experiment_id } => status::run(system, experiment_id),
            ExperimentCommand::Watch { id: experiment_id } => watch::run(system, experiment_id),
        }
    }
}