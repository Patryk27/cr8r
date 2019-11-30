// @todo ensure controller's protocol version matches ours

#![feature(try_blocks)]
#![feature(type_ascription)]

use std::path::PathBuf;
use std::process::exit;

use colored::Colorize;
use structopt::StructOpt;

pub use self::{
    commands::*,
    core::*,
    system::*,
};

mod commands;
mod core;
mod system;

#[tokio::main]
async fn main() {
    let cmd = Command::from_args();

    let config = config::load(
        &PathBuf::from("client.yaml")
    ).unwrap(); // @todo error handling

    let system = System::new(config);

    match cmd.run(system).await {
        Ok(_) => (),

        Err(err) => {
            println!("{} {}", "Error:".red(), err);

            exit(2);
        }
    }
}
