use anyhow::Result;

use lib_interop::convert;
use lib_interop::proto::controller::PFindRunnersRequest;
use lib_ui::spinner;

use crate::{System, ui};

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", lib_ui::Header::new("Runners"));

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