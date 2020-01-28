use anyhow::*;
use structopt::StructOpt;

use crate::app::AppContext;
use crate::controller::ControllerCommand;
use crate::experiment::ExperimentCommand;

#[derive(Debug, StructOpt)]
pub enum AppCommand {
    Controller(ControllerCommand),
    Experiment(ExperimentCommand),
}

impl AppCommand {
    pub async fn run(self, ctxt: &mut AppContext) -> Result<()> {
        match self {
            AppCommand::Controller(cmd) => {
                cmd.run(ctxt).await
            }

            AppCommand::Experiment(cmd) => {
                cmd.run(ctxt).await
            }
        }
    }
}