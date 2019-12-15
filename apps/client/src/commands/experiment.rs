use structopt::StructOpt;

use crate::{Result, System};

pub use self::launch::*;

mod abort;
mod launch;
mod show;
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

    Show {
        #[structopt(short = "r", long = "show-reports")]
        show_reports: bool,

        id: String,
    },

    Watch {
        id: String,
    },
}

impl ExperimentCommand {
    pub async fn run(self, system: System) -> Result<()> {
        match self {
            ExperimentCommand::Abort { id } => {
                abort::run(system, id).await
            }

            ExperimentCommand::Launch { watch, cmd } => {
                cmd.run(system, watch).await
            }

            ExperimentCommand::Show { id, show_reports: report } => {
                show::run(system, &id, report).await
            }

            ExperimentCommand::Watch { id } => {
                watch::run(system, id).await
            }
        }
    }
}