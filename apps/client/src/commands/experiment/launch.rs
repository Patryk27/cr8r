use colored::Colorize;
use structopt::StructOpt;

use lib_protocol::controller::{LaunchExperimentRequest, LaunchTrySystemExperiment, LaunchTryToolchainExperiment};
use lib_protocol::controller::launch_experiment_reply::Status;
use lib_protocol::controller::launch_experiment_request::Experiment;

use crate::{Result, System};

#[derive(Debug, StructOpt)]
pub enum LaunchExperimentCommand {
    #[structopt(name = "try-system")]
    TrySystem {
        system: String,
    },

    #[structopt(name = "try-toolchain")]
    TryToolchain {
        toolchain: String,
    },
}

impl LaunchExperimentCommand {
    pub async fn run(self, system: System) -> Result<()> {
        run(system, match self {
            LaunchExperimentCommand::TrySystem { system } => {
                Experiment::TrySystem(LaunchTrySystemExperiment { system })
            }

            LaunchExperimentCommand::TryToolchain { toolchain } => {
                Experiment::TryToolchain(LaunchTryToolchainExperiment { toolchain })
            }
        }).await
    }
}

async fn run(mut system: System, experiment: Experiment) -> Result<()> {
    let request = LaunchExperimentRequest { experiment: Some(experiment) };

    let reply = system
        .client().await?
        .launch_experiment(request).await?
        .into_inner();

    match reply.status.unwrap() {
        Status::Success(reply) => {
            println!("{}", "Success!".green());
            println!();
            println!("Experiment `{}` has been created.", reply.experiment_id.blue());
            println!("It\'s now waiting for a runner to pick it up.");
            println!();
            println!("You can see status of your experiment using:");
            println!("$ {}", format!("cr8r experiment status {}", reply.experiment_id).green());
        }

        _ => unimplemented!(),
    }

    Ok(())
}