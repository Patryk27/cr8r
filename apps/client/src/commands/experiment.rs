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
        id: String,

        #[structopt(short = "d", long = "detailed")]
        detailed: bool,

        #[structopt(short = "j", long = "show-jobs")]
        show_jobs: bool,

        #[structopt(short = "r", long = "show-reports")]
        show_reports: bool,
    },

    Watch {
        id: String,
    },
}

impl ExperimentCommand {
    pub async fn run(self, system: System) -> Result<()> {
        match self {
            ExperimentCommand::Abort { id } => {
                abort::abort(system, id)
                    .await
            }

            ExperimentCommand::Launch { watch, cmd } => {
                cmd.run(system, watch)
                    .await
            }

            ExperimentCommand::Show { id, detailed, show_jobs, show_reports } => {
                show::show(system, &id, detailed || show_jobs, detailed || show_reports)
                    .await
            }

            ExperimentCommand::Watch { id } => {
                watch::watch(system, id)
                    .await
            }
        }
    }
}