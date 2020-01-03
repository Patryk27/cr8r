use anyhow::Result;
use structopt::StructOpt;

use crate::System;

mod status;

#[derive(Debug, StructOpt)]
pub enum ControllerCommand {
    Status,
}

impl ControllerCommand {
    pub async fn run(self, system: System) -> Result<()> {
        match self {
            ControllerCommand::Status => {
                status::status(system)
                    .await
            }
        }
    }
}