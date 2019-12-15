use structopt::StructOpt;

use crate::{Result, System};

mod status;

#[derive(Debug, StructOpt)]
pub enum ControllerCommand {
    Status,
}

impl ControllerCommand {
    pub async fn run(self, system: System) -> Result<()> {
        match self {
            ControllerCommand::Status => {
                status::run(system).await
            }
        }
    }
}