use colored::Colorize;

use lib_protocol::for_client::PFindRunnersRequest;

use crate::{Result, spinner, System, ui};

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", "# Runners".blue());
    println!();

    let runners = spinner! {
        system
            .client().await?
            .find_runners(PFindRunnersRequest::default()).await?
            .into_inner()
            .runners
    };

    println!("{}", ui::RunnersTable::new(&runners));

    Ok(())
}