use std::time::Duration;

use anyhow::Result;
use colored::Colorize;

use lib_ui::spinner;

use crate::System;

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", lib_ui::Header::new("Controller"));

    println!(
        "Address: {}",
        system.config().controller.address.green(),
    );

    let status = spinner! {
        system
            .client()
            .await?
            .howdy()
            .await?
    };

    // @todo extract it to a separate component

    println!(
        "Version: {}",
        status.version.to_string().green(),
    );

    println!(
        "Uptime: {}",
        format!("{:?}", Duration::from_secs(status.uptime)).green(),
    );

    Ok(())
}