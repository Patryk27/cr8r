use anyhow::Result;

use lib_interop::convert;
use lib_interop::proto::controller::PFindRunnersRequest;

use crate::{spinner, System, ui};

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", ui::Header::new("Runners"));

    let runners = spinner! {
        system
            .client()
            .await?
            .find_runners(PFindRunnersRequest::default())
            .await?
            .runners
    };

    let runners = convert!(runners as [_?]);

    print!("{}", ui::RunnersTable::new(&runners));

    Ok(())
}