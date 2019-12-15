use colored::Colorize;

use lib_protocol::for_client::PFindExperimentsRequest;

use crate::{Result, spinner, System, ui};

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", "# Experiments".blue());
    println!();

    let experiments = spinner! {
        system
            .client().await?
            .find_experiments(PFindExperimentsRequest::default()).await?
            .into_inner()
            .experiments
    };

    println!("{}", ui::ExperimentsTable::new(&experiments));

    Ok(())
}
