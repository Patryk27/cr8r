use anyhow::{Context, Result};
use colored::Colorize;

use lib_ui::{Message, spinner};

use crate::System;

pub use self::definition::*;

mod definition;

pub async fn launch(mut system: System, watch: bool, definition: Definition) -> Result<()> {
    let definition = definition.parse()
        .context("Could not parse experiment's definition")?;

    let id = spinner! {
        system
            .client()
            .await?
            .create_experiment(definition.into())
            .await?
            .id
    };

    println!("{}", Message::success(
        "Success:",
        [
            format!("Experiment `{}` has been created.", id.blue()),
            "It's now waiting for a runner to pick it up.".to_string(),
        ].join("\n"),
    ));

    if watch {
        super::watch::watch(system, id)
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
