use colored::Colorize;
use indicatif::ProgressBar;

use lib_protocol::client::WatchExperimentRequest;

use crate::{Result, System};

pub async fn run(mut system: System, experiment_id: String) -> Result<()> {
    println!("Attaching to experiment `{}`.", experiment_id.blue());

    let mut reply = system
        .client().await?
        .watch_experiment(WatchExperimentRequest { experiment_id }).await?
        .into_inner();

    println!("Attached, logs follow:");
    println!();

    loop {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(100);

        let reply = reply.message().await?;

        pb.finish_and_clear();

        if let Some(reply) = reply {
            println!("{}", reply.line);
        } else {
            break;
        }
    }

    Ok(())
}