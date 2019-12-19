use std::time::Duration;

use colored::Colorize;

use lib_protocol::for_client::PHelloRequest;

use crate::{Result, spinner, System, ui};

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", ui::Header::new("Controller"));
    println!();

    println!(
        "Address: {}",
        system.config().controller.address.green(),
    );

    let status = spinner! {
        system
            .client().await?
            .hello(PHelloRequest::default()).await?
            .into_inner()
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