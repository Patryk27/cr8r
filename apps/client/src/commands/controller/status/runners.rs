use anyhow::*;

use lib_core_ui::*;
use lib_interop::convert;
use lib_interop::proto::controller::PFindRunnersRequest;

use crate::modules::app::AppContext;
use crate::widgets::RunnerListWidget;

pub async fn print(ctxt: &mut AppContext) -> Result<()> {
    println!("{}", HeaderWidget::new("Runners"));

    let runners = spinner! {
        ctxt
            .client()
            .await?
            .find_runners(PFindRunnersRequest::default())
            .await?
            .runners
    };

    let runners = convert!(runners as [_?]);

    print!("{}", RunnerListWidget::new(&runners));

    Ok(())
}