use std::convert::TryInto;

use anyhow::*;
use tokio::stream::StreamExt;

use lib_core_ui::*;
use lib_interop::proto::core::PExperimentId;

use crate::modules::app::AppContext;
use crate::modules::experiment::ExperimentWatcher;
use crate::widgets::InlineReportWidget;

pub async fn watch(ctxt: &mut AppContext, id: PExperimentId) -> Result<()> {
    let id = id
        .try_into()
        .unwrap();

    let mut reports = spinner! {
        ExperimentWatcher::new(ctxt)
            .watch(id)
            .await?
    };

    println!("Attached to experiment, logs follow:");
    println!();

    while let Some(report) = spinner! { reports.next().await } {
        println!("{}", InlineReportWidget::new(&report?));
    }

    println!();
    println!("Experiment's stream closed");

    Ok(())
}
