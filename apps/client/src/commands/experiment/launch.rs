use colored::Colorize;
use structopt::StructOpt;

use lib_protocol::core::p_experiment_def::*;
use lib_protocol::core::PExperimentDef;
use lib_protocol::for_client::PLaunchExperimentRequest;

use crate::{Result, spinner, System};

#[derive(Debug, StructOpt)]
pub enum LaunchExperimentCommand {
    #[structopt(name = "try-toolchain")]
    TryToolchain {
        toolchain: String,
    },
}

impl LaunchExperimentCommand {
    pub async fn run(self, system: System, watch: bool) -> Result<()> {
        launch_experiment(system, watch, match self {
            LaunchExperimentCommand::TryToolchain { toolchain } => {
                Op::TryToolchain(PTryToolchain { toolchain })
            }
        }).await
    }
}

async fn launch_experiment(
    mut system: System,
    watch: bool,
    experiment: Op,
) -> Result<()> {
    let experiment_def = Some(PExperimentDef {
        op: Some(experiment),
    });

    let reply = spinner! {
        system
            .client().await?
            .launch_experiment(PLaunchExperimentRequest { experiment_def }).await?
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