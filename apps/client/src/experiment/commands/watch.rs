use anyhow::*;

use lib_core_ui::*;
use lib_interop::convert;
use lib_interop::proto::core::PExperimentId;

use crate::app::AppContext;
use crate::report::InlineReportWidget;

pub async fn watch(ctxt: &mut AppContext, id: PExperimentId) -> Result<()> {
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
