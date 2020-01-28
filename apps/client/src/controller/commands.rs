use anyhow::*;
use structopt::StructOpt;

use crate::app::AppContext;

mod status;

#[derive(Debug, StructOpt)]
pub enum ControllerCommand {
    Status,
}

impl ControllerCommand {
    pub async fn run(self, ctxt: &mut AppContext) -> Result<()> {
        match self {
            ControllerCommand::Status => {
                status::status(ctxt).await
            }
        }
    }
}
