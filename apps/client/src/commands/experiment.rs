use anyhow::*;
use structopt::StructOpt;

use crate::System;

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
        definition: launch::Definition,
    },

    Show {
        id: String,

        #[structopt(short = "a", long = "show-all")]
        show_all: bool,

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

            ExperimentCommand::Launch { watch, definition } => {
                launch::launch(system, watch, definition)
                    .await
            }

            ExperimentCommand::Show { id, show_all, show_jobs, show_reports } => {
                show::show(system, &id, show_all || show_jobs, show_all || show_reports)
                    .await
            }

            ExperimentCommand::Watch { id } => {
                watch::watch(system, id)
                    .await
            }
        }
    }
}