use std::time::Duration;

use colored::Colorize;

use lib_protocol::client::HelloRequest;

use crate::{Result, System};

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", "# Controller".blue());
    println!();

    println!("Address: {}", system.config().controller.address.green());

    let status = system
        .client().await?
        .hello(HelloRequest {}).await?
        .into_inner();

    println!("Version: {}", format!("{}", status.version).green());
    println!("Uptime: {}", format!("{:?}", Duration::from_secs(status.uptime)).green());

    Ok(())
}