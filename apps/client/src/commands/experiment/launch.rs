use anyhow::{Context, Result};
use colored::Colorize;
use structopt::StructOpt;

use crate::{spinner, System};

pub use self::definition::*;

mod definition;

pub fn launch(system: System, watch: bool, definition: Definition) -> Result<()> {
    let definition = definition.parse()
        .context("Could not parse experiment's definition")?;

    panic!("{:#?}", definition);
}

//async fn launch_experiment(
//    mut system: System,
//    definition: DExperimentDefinition,
//    watch: bool,
//) -> Result<()> {
//    let id = spinner! {
//        system
//            .client()
//            .await?
//            .create_experiment(definition.into())
//            .await?
//            .id
//    };
//
//    println!("{}", "Success!".green());
//    println!("Your experiment has been created and it\'s now waiting for a runner.");
//    println!();
//
//    if watch {
//        super::watch::watch(system, id)
//            .await?;
//    } else {
//        println!("You can see status of your experiment using:");
//        println!("$ {}", format!("cr8r experiment status {}", id).blue());
//        println!();
//        println!("Or, if you prefer a semi-real-time view:");
//        println!("$ {}", format!("cr8r experiment watch {}", id).blue());
//    }
//
//    Ok(())
//}