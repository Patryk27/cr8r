use anyhow::*;

use lib_interop::convert;
use lib_ui::*;

use crate::app::AppContext;
use crate::report::InlineReportWidget;

pub async fn watch(ctxt: &mut AppContext, id: String) -> Result<()> {
    let mut reply = spinner! {
        ctxt.client()
            .await?
            .watch_experiment(id)
            .await?
    };

    println!("Attached to experiment, logs follow:");
    println!();

    while let Some(report) = spinner! { reply.message().await? } {
        let report = convert!(report as _?);
        println!("{}", InlineReportWidget::new(&report));
    }

    println!();
    println!("Experiment's stream closed");

    Ok(())
}
