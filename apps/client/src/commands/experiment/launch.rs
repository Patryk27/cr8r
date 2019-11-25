use colored::Colorize;
use structopt::StructOpt;

use lib_protocol_core::ExperimentDefinition;

use crate::{Result, System};

#[derive(Debug, StructOpt)]
pub enum LaunchExperimentCommand {
    #[structopt(name = "try-os")]
    TryOs {
        os: String,
    },

    #[structopt(name = "try-toolchain")]
    TryToolchain {
        toolchain: String,
    },
}

impl LaunchExperimentCommand {
    pub fn run(self, system: System) -> Result<()> {
        run(system, match self {
            LaunchExperimentCommand::TryOs { os } => {
                ExperimentDefinition::TryOs { os }
            }

            LaunchExperimentCommand::TryToolchain { toolchain } => {
                ExperimentDefinition::TryToolchain { toolchain }
            }
        })
    }
}

fn run(system: System, experiment: ExperimentDefinition) -> Result<()> {
    let id = system
        .connector()
        .launch_experiment(experiment)?;

    println!("{}", "Success!".green());
    println!();
    println!("Experiment `{}` has been created.", id.to_string().blue());
    println!("It\'s now waiting for a runner to pick it up.");
    println!();
    println!("You can see status of your experiment using:");
    println!("$ {}", format!("cr8r experiment status {}", id).green());

    Ok(())
}