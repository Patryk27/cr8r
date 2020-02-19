use anyhow::*;

use lib_core_ui::*;

use crate::modules::app::AppContext;
use crate::widgets::RunnerListWidget;

pub async fn print(ctxt: &mut AppContext) -> Result<()> {
    HeaderWidget::new("Runners")
        .println();

    let runners = spinner! {
        ctxt.conn().await?
            .runners()
            .find_many().await?
    };

    RunnerListWidget::new(&runners)
        .print();

    Ok(())
}