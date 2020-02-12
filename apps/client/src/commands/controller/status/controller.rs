use std::time::Duration;

use anyhow::*;
use colored::Colorize;

use lib_core_ui::*;

use crate::modules::app::AppContext;

pub async fn print(ctxt: &mut AppContext) -> Result<()> {
    HeaderWidget::new("Controller")
        .println();

    println!(
        "Address: {}",
        ctxt.config().controller.address.green(),
    );

    let status = spinner! {
        ctxt.client()
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