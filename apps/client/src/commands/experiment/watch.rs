use anyhow::Result;
use colored::Colorize;

use lib_interop::convert;

use crate::{spinner, System, ui};

pub async fn watch(mut system: System, id: String) -> Result<()> {
    let mut reply = spinner! {
        system
            .client()
            .await?
            .watch_experiment(id)
            .await?
    };

    println!("Attached, logs follow:");
    println!();

    while let Some(report) = spinner! { reply.message().await? } {
        let report = convert!(report as _?);
        println!("{}", ui::InlineReport::new(&report));
    }

    println!();
    println!("Stream closed");

    Ok(())
}
