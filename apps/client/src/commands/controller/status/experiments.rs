use anyhow::*;

use lib_core_ui::*;
use lib_interop::convert;
use lib_interop::proto::controller::PFindExperimentsRequest;

use crate::modules::app::AppContext;
use crate::widgets::ExperimentListWidget;

pub async fn print(ctxt: &mut AppContext) -> Result<()> {
    println!("{}", HeaderWidget::new("Experiments"));

    let experiments = spinner! {
        ctxt.client()
            .await?
            .find_experiments(PFindExperimentsRequest::default())
            .await?
            .experiments
    };

    let experiments = convert!(experiments as [_?]);

    print!("{}", ExperimentListWidget::new(&experiments));

    Ok(())
}
