use colored::Colorize;
use structopt::StructOpt;

use lib_protocol::client::*;
use lib_protocol::core::{experiment_definition, ExperimentDefinition};
use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

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
    pub async fn run(self, system: System, watch: bool) -> Result<()> {
        run(system, watch, match self {
            LaunchExperimentCommand::TrySystem { system } => {
                ExperimentDefinitionInner::TrySystem(experiment_definition::TrySystem {
                    system,
                })
            }

            LaunchExperimentCommand::TryToolchain { toolchain } => {
                ExperimentDefinitionInner::TryToolchain(experiment_definition::TryToolchain {
                    toolchain,
                })
            }
        }).await
    }
}

async fn run(
    mut system: System,
    watch: bool,
    experiment: ExperimentDefinitionInner,
) -> Result<()> {
    let request = LaunchExperimentRequest {
        experiment: Some(ExperimentDefinition {
            experiment_definition_inner: Some(experiment),
        }),
    };

    let reply = system
        .client().await?
        .launch_experiment(request).await?
        .into_inner();

    println!("{}", "Success!".green());
    println!();
    println!("Your experiment has been created and it\'s now waiting for a runner.");
    println!();

    if watch {
        super::watch::run(system, reply.id).await?;
    } else {
        println!("You can see status of your experiment using:");
        println!("$ {}", format!("cr8r experiment status {}", reply.id).blue());
        println!();
        println!("Or, if you prefer a semi-real-time view:");
        println!("$ {}", format!("cr8r experiment watch {}", reply.id).blue());
    }

    Ok(())
}