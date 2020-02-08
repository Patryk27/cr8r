use anyhow::*;
use structopt::StructOpt;

use crate::modules::app::AppContext;

pub use self::{
    controller::*,
    experiment::*,
};

mod controller;
mod experiment;

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