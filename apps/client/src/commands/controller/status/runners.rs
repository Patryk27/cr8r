use anyhow::*;

use lib_core_ui::*;
use lib_interop::convert;
use lib_interop::proto::services::PFindRunnersRequest;

use crate::modules::app::AppContext;
use crate::widgets::RunnerListWidget;

pub async fn print(ctxt: &mut AppContext) -> Result<()> {
    HeaderWidget::new("Runners")
        .println();

    let runners = spinner! {
        ctxt.runners()
            .await?
            .find_runners(PFindRunnersRequest::default())
            .await?
            .into_inner()
            .runners
    };

    let runners = convert!(runners as [_?]);

    RunnerListWidget::new(&runners)
        .print();

    Ok(())
}