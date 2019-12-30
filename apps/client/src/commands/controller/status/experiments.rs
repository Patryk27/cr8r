use lib_interop::convert;
use lib_interop::protocol::for_client::PFindExperimentsRequest;

use crate::{Result, spinner, System, ui};

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", ui::Header::new("Experiments"));

    let experiments = spinner! {
        system
            .client().await?
            .find_experiments(PFindExperimentsRequest::default()).await?
            .into_inner()
            .experiments
    };

    let experiments = convert!(experiments as [_?]);

    print!("{}", ui::ExperimentsTable::new(&experiments));

    Ok(())
}
