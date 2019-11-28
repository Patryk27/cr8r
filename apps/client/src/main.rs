// @todo ensure controller's protocol version matches ours

#![feature(try_blocks)]
#![feature(type_ascription)]

use std::process::exit;

use colored::Colorize;
use structopt::StructOpt;

pub use self::{
    commands::*,
    error::{Result, StdResult},
    system::*,
};

mod commands;
mod error;
mod system;

#[tokio::main]
async fn main() {
    let cmd = Command::from_args();

    // Initialize the system
    let system = match System::new() {
        Ok(system) => system,

        Err(err) => {
            println!("{}: Failed to initialize the application.", "Error".red());
            println!("Please make sure the `{}` file exists and has a correct structure.", "client.yaml".green());
            println!();
            println!("{} {}", "Caused by:".red(), err);

            exit(1);
        }
    };

    // Takeoff!
    match cmd.run(system).await {
        Ok(_) => (),

        Err(err) => {
            println!("{} {}", "Error:".red(), err);

            exit(2);
        }
    }
}
