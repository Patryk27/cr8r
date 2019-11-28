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
    pub async fn run(self, system: System) -> Result<()> {
        match self {
            ExperimentCommand::Abort { id: experiment_id } => abort::run(system, experiment_id).await,
            ExperimentCommand::Report { id: experiment_id } => report::run(system, experiment_id).await,
            ExperimentCommand::Launch(cmd) => cmd.run(system).await,
            ExperimentCommand::Status { id: experiment_id } => status::run(system, experiment_id).await,
            ExperimentCommand::Watch { id: experiment_id } => watch::run(system, experiment_id).await,
        }
    }
}