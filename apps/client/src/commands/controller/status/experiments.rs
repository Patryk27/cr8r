use anyhow::*;

use lib_core_ui::*;

use crate::modules::app::AppContext;
use crate::widgets::ExperimentListWidget;

pub async fn print(ctxt: &mut AppContext) -> Result<()> {
    HeaderWidget::new("Experiments")
        .println();

    let experiments = spinner! {
        ctxt.conn()
            .await?
            .experiments()
            .find_many()
            .await?
    };

    ExperimentListWidget::new(&experiments)
        .print();

    Ok(())
}
