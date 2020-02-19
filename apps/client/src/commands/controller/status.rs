use anyhow::*;

use crate::modules::app::AppContext;

mod experiments;
mod controller;
mod runners;

pub async fn status(ctxt: &mut AppContext) -> Result<()> {
    controller::print(ctxt).await?;

    println!();

    experiments::print(ctxt).await?;

    println!();

    runners::print(ctxt).await?;

    Ok(())
}
