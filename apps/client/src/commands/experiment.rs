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

    Launch {
        #[structopt(short = "w", long = "watch")]
        watch: bool,

        #[structopt(flatten)]
        cmd: LaunchExperimentCommand,
    },

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
            ExperimentCommand::Abort { id } => abort::run(system, id).await,
            ExperimentCommand::Launch { watch, cmd } => cmd.run(system, watch).await,
            ExperimentCommand::Report { id } => report::run(system, id).await,
            ExperimentCommand::Status { id } => status::run(system, id).await,
            ExperimentCommand::Watch { id } => watch::run(system, id).await,
        }
    }
}