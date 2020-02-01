use anyhow::*;
use colored::Colorize;

use lib_core_ui::*;

use crate::app::AppContext;
use crate::definition::DefinitionArg;

pub async fn launch(ctxt: &mut AppContext, watch: bool, definition: DefinitionArg) -> Result<()> {
    let definition = definition
        .parse()
        .context("Could not understand experiment")?;

    let id = spinner! {
        ctxt.client()
            .await?
            .create_experiment(definition.into())
            .await?
            .id
    };

    println!("{}", MessageWidget::success(
        "Success:",
        [
            format!("Experiment `{}` has been created.", id.to_string().blue()),
            "It's now waiting for a runner to pick it up.".to_string(),
        ].join("\n"),
    ));

    if watch {
        super::watch::watch(ctxt, id)
            .await?;
    } else {
        println!("You can see status of your experiment using:");
        println!("$ {}", format!("cr8r experiment status {}", id).blue());
        println!();
        println!("Or, if you prefer a real-time view:");
        println!("$ {}", format!("cr8r experiment watch {}", id).blue());
    }

    Ok(())
}
