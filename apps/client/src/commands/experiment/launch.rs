use colored::Colorize;
use structopt::StructOpt;

use lib_interop::contract::CExperimentDefinition;
use lib_interop::protocol::for_client::PCreateExperimentRequest;

use crate::{Result, spinner, System};

#[derive(Debug, StructOpt)]
pub enum LaunchExperimentCommand {
    #[structopt(name = "override-toolchain")]
    OverrideToolchain {
        toolchain: String,
    },

    #[structopt(name = "override-crate")]
    OverrideCrate {
        name: String,
        version: String,
    },
}

impl LaunchExperimentCommand {
    pub async fn run(self, system: System, watch: bool) -> Result<()> {
        use LaunchExperimentCommand::*;

        launch_experiment(system, match self {
            OverrideToolchain { toolchain } => {
                CExperimentDefinition::OverrideToolchain { toolchain }
            }

            OverrideCrate { name, version } => {
                CExperimentDefinition::OverrideCrate { name, version }
            }
        }, watch).await
    }
}

async fn launch_experiment(
    mut system: System,
    definition: CExperimentDefinition,
    watch: bool,
) -> Result<()> {
    let reply = spinner! {
        system
            .client()
            .await?
            .create_experiment(PCreateExperimentRequest { experiment_definition: Some(definition.into()) })
            .await?
            .into_inner()
    };

    println!("{}", "Success!".green());
    println!("Your experiment has been created and it\'s now waiting for a runner.");
    println!();

    if watch {
        super::watch::watch(system, reply.id)
            .await?;
    } else {
        println!("You can see status of your experiment using:");
        println!("$ {}", format!("cr8r experiment status {}", reply.id).blue());
        println!();
        println!("Or, if you prefer a semi-real-time view:");
        println!("$ {}", format!("cr8r experiment watch {}", reply.id).blue());
    }

    Ok(())
}