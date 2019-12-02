use lib_protocol::client::WatchExperimentRequest;

use crate::{Result, System};

pub async fn run(mut system: System, experiment_id: String) -> Result<()> {
    let mut reply = system
        .client().await?
        .watch_experiment(WatchExperimentRequest { experiment_id }).await?
        .into_inner();

    while let Some(report) = reply.message().await? {
        println!("{:#?}", report);
    }

    Ok(())
}