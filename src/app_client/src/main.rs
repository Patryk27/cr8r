#![feature(type_ascription)]

use std::path::PathBuf;

use structopt::StructOpt;

use self::{
    error::Result,
    http::Client,
};

mod cli;
pub mod config;
mod error;
mod http;

// @todo validate for controller version mismatch

fn main() -> Result<()> {
    match cli::Command::from_args() {
        cli::Command::Controller(cmd) => {
            run_controller_cmd(cmd)
        }

        cli::Command::Experiment(cmd) => {
            run_experiment_cmd(cmd)
        }
    }
}

fn run_controller_cmd(cmd: cli::command::Controller) -> Result<()> {
    match cmd {
        cli::command::Controller::Status => {
            match client()?.controller_status() {
                Ok(status) => {
                    println!("{:#?}", status);

                    Ok(())
                }

                Err(err) => {
                    println!("Could not get controller\'s status: {:?}", err);

                    Err(err)
                }
            }
        }
    }
}

fn run_experiment_cmd(cmd: cli::command::Experiment) -> Result<()> {
    match cmd {
        cli::command::Experiment::Report { experiment_id: _ } => {
            unimplemented!()
        }

        cli::command::Experiment::Run(experiment) => {
            match client()?.create_experiment(experiment) {
                Ok(id) => {
                    println!("Success!");
                    println!();
                    println!("Your experiment `{}` has been created.", id);
                    println!("It\'s now waiting for a runner to pick it up.");
                    println!();
                    println!("You can see status of your experiment using:");
                    println!("  cr8r experiment status {}", id);

                    Ok(())
                }

                Err(err) => {
                    println!("Could not create experiment: {:?}", err);

                    Err(err)
                }
            }
        }

        cli::command::Experiment::Abort { experiment_id: _ } => {
            unimplemented!()
        }
    }
}

fn client() -> Result<Client> {
    let config = config::from_file(
        &PathBuf::from("client.yaml")
    )?;

    Client::new(config.controller.address, config.controller.secret)
}