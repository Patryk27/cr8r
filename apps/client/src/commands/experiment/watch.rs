use colored::Colorize;

use lib_protocol::client::WatchExperimentRequest;

use crate::{Result, System};

pub async fn run(mut system: System, experiment_id: String) -> Result<()> {
    println!("Attaching to experiment `{}`", experiment_id.blue());

    let mut reply = system
        .client().await?
        .watch_experiment(WatchExperimentRequest { experiment_id }).await?
        .into_inner();

    println!("Attached, stdout follows:");
    println!();

    while let Some(reply) = reply.message().await? {
        println!("{}", reply.line);
    }

    Ok(())
}