use anyhow::Result;
use colored::Colorize;
use structopt::StructOpt;

use lib_interop::domain::DExperimentDefinition;

use crate::{spinner, System};

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
                DExperimentDefinition::OverrideToolchain { toolchain }
            }

            OverrideCrate { name, version } => {
                DExperimentDefinition::OverrideCrate { name, version }
            }
        }, watch).await
    }
}

async fn launch_experiment(
    mut system: System,
    definition: DExperimentDefinition,
    watch: bool,
) -> Result<()> {
    let id = spinner! {
        system
            .client()
            .await?
            .create_experiment(definition.into())
            .await?
            .id
    };

    println!("{}", "Success!".green());
    println!("Your experiment has been created and it\'s now waiting for a runner.");
    println!();

    if watch {
        super::watch::watch(system, id)
            .await?;
    } else {
        println!("You can see status of your experiment using:");
        println!("$ {}", format!("cr8r experiment status {}", id).blue());
        println!();
        println!("Or, if you prefer a semi-real-time view:");
        println!("$ {}", format!("cr8r experiment watch {}", id).blue());
    }

    Ok(())
}