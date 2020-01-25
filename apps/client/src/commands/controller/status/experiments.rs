use anyhow::*;

use lib_interop::convert;
use lib_interop::proto::controller::PFindExperimentsRequest;
use lib_ui::spinner;

use crate::{System, ui};

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", lib_ui::Header::new("Experiments"));

    let experiments = spinner! {
        system
            .client()
            .await?
            .find_experiments(PFindExperimentsRequest::default())
            .await?
            .experiments
    };

    let experiments = convert!(experiments as [_?]);

    print!("{}", ui::ExperimentsTable::new(&experiments));

    Ok(())
}
