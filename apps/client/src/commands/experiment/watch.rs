use std::convert::TryInto;

use anyhow::*;
use tokio::stream::StreamExt;

use lib_core_ui::*;
use lib_interop::proto::models::PExperimentId;

use crate::modules::app::AppContext;
use crate::widgets::InlineReportWidget;

pub async fn watch(ctxt: &mut AppContext, id: PExperimentId) -> Result<()> {
    let id = id
        .try_into()
        .unwrap();

    let mut reports = spinner! {
        ctxt.conn()
            .await?
            .experiments()
            .watch(id)
            .await?
    };

    println!("Attached to experiment, logs follow:");
    println!();

    while let Some(report) = spinner! { reports.next().await } {
        InlineReportWidget::new(&report?)
            .println();
    }

    println!();
    println!("Experiment's stream closed");

    Ok(())
}
