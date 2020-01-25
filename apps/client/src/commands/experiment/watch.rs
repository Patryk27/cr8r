use anyhow::*;

use lib_interop::convert;
use lib_ui::spinner;

use crate::{System, ui};

pub async fn watch(mut system: System, id: String) -> Result<()> {
    let mut reply = spinner! {
        system
            .client()
            .await?
            .watch_experiment(id)
            .await?
    };

    println!("Attached to experiment, logs follow:");
    println!();

    while let Some(report) = spinner! { reply.message().await? } {
        let report = convert!(report as _?);
        println!("{}", ui::InlineReport::new(&report));
    }

    println!();
    println!("Experiment's stream closed");

    Ok(())
}
