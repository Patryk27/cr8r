use anyhow::*;
use structopt::StructOpt;

use lib_interop::proto::models::PExperimentId;

use crate::modules::app::AppContext;
use crate::modules::definition::DefinitionArg;

mod delete;
mod launch;
mod list;
mod show;
mod stop;
mod watch;

#[derive(Debug, StructOpt)]
pub enum ExperimentCommand {
    Delete {
        id: PExperimentId,
    },

    Launch {
        #[structopt(short = "w", long = "watch")]
        watch: bool,

        #[structopt(flatten)]
        definition: DefinitionArg,
    },

    Show {
        id: PExperimentId,

        #[structopt(short = "s", long = "show-all")]
        show_all: bool,

        #[structopt(short = "a", long = "show-attachments")]
        show_attachments: bool,

        #[structopt(short = "j", long = "show-jobs")]
        show_jobs: bool,

        #[structopt(short = "r", long = "show-reports")]
        show_reports: bool,
    },

    Stop {
        id: PExperimentId,
    },

    Watch {
        id: PExperimentId,
    },
}

impl ExperimentCommand {
    pub async fn run(self, ctxt: &mut AppContext) -> Result<()> {
        match self {
            ExperimentCommand::Delete { id } => {
                delete::delete(ctxt, id).await
            }

            ExperimentCommand::Launch { watch, definition } => {
                launch::launch(ctxt, watch, definition).await
            }

            ExperimentCommand::Show { id, show_all, show_attachments, show_jobs, show_reports } => {
                show::show(
                    ctxt,
                    id,
                    show_all || show_attachments,
                    show_all || show_jobs,
                    show_all || show_reports,
                ).await
            }

            ExperimentCommand::Stop { id } => {
                stop::stop(ctxt, id).await
            }

            ExperimentCommand::Watch { id } => {
                watch::watch(ctxt, id).await
            }
        }
    }
}
