use colored::Colorize;

use lib_interop::convert;
use lib_interop::protocol::for_client::PWatchExperimentRequest;

use crate::{Result, spinner, System, ui};

pub async fn watch(mut system: System, id: String) -> Result<()> {
    println!("Attaching to experiment `{}`", id.cyan());

    let mut reply = spinner! {
        system
            .client()
            .await?
            .watch_experiment(PWatchExperimentRequest { id })
            .await?
            .into_inner()
    };

    println!("Attached, logs follow:");
    println!();

    while let Some(report) = spinner! { reply.message().await? } {
        let report = convert!(report as _?);

        let report = ui::InlineReport::new(&report)
            .to_string();

        if !report.is_empty() {
            println!("{}", report);
        }
    }

    println!();
    println!("Stream closed");

    Ok(())
}
