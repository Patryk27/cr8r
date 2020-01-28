use anyhow::*;
use structopt::StructOpt;

use crate::app::AppContext;
use crate::definition::DefinitionArg;

mod abort;
mod launch;
mod list;
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
        definition: DefinitionArg,
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
    pub async fn run(self, ctxt: &mut AppContext) -> Result<()> {
        match self {
            ExperimentCommand::Abort { id } => {
                abort::abort(ctxt, id).await
            }

            ExperimentCommand::Launch { watch, definition } => {
                launch::launch(ctxt, watch, definition).await
            }

            ExperimentCommand::Show { id, show_all, show_jobs, show_reports } => {
                show::show(ctxt, &id, show_all || show_jobs, show_all || show_reports).await
            }

            ExperimentCommand::Watch { id } => {
                watch::watch(ctxt, id).await
            }
        }
    }
}
