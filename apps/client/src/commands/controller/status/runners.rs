use colored::Colorize;
use prettytable::*;

use lib_protocol::core::p_runner::p_status::*;
use lib_protocol::for_client::PFindRunnersRequest;

use crate::{Result, System};

use super::table;

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", "# Runners".blue());
    println!();

    let runners = system
        .client().await?
        .find_runners(PFindRunnersRequest {}).await?
        .into_inner()
        .runners;

    if runners.is_empty() {
        println!("There are no runners registered");
        return Ok(());
    }

    let mut table = table(row![
        "Name", "Status", "Joined at", "Heartbeaten at",
    ]);

    for runner in runners {
        let status = match runner.status.unwrap().op.unwrap() {
            Op::Idle(PIdle { since }) => {
                format!(
                    "{} (since {})",
                    "idle".purple(),
                    since,
                )
            }

            Op::Working(PWorking { since, .. }) => {
                format!(
                    "{} (since {})",
                    "working".green(),
                    since,
                )
            }

            Op::Zombie(PZombie { since }) => {
                format!(
                    "{} (since {})",
                    "zombie".red(),
                    since,
                )
            }
        };

        table.add_row(row![
            runner.name.bright_cyan(),
            status,
            runner.joined_at,
            runner.heartbeaten_at,
        ]);
    }

    table.printstd();

    Ok(())
}