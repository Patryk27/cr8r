use colored::Colorize;
use prettytable::*;

use lib_protocol::client::FindRunnersRequest;
use lib_protocol::core::runner::status;

use crate::{Result, System};

use super::table;

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", "# Runners".blue());
    println!();

    let runners = system
        .client().await?
        .find_runners(FindRunnersRequest {}).await?
        .into_inner()
        .runners;

    if runners.is_empty() {
        println!("There are no runners registered.");
        return Ok(());
    }

    let mut table = table(row![
        "Id", "Name", "Status", "Joined at", "Heartbeaten at",
    ]);

    for runner in runners {
        let status = match runner.status.unwrap().op.unwrap() {
            status::Op::Idle(status::Idle { since }) => {
                format!(
                    "{} (since {})",
                    "idle".purple(),
                    since,
                )
            }

            status::Op::Working(status::Working { since, .. }) => {
                format!(
                    "{} (since {})",
                    "working".green(),
                    since,
                )
            }

            status::Op::Zombie(status::Zombie { since }) => {
                format!(
                    "{} (since {})",
                    "zombie".red(),
                    since,
                )
            }
        };

        table.add_row(row![
            runner.id.bright_cyan(),
            runner.name,
            status,
            runner.joined_at,
            runner.heartbeaten_at,
        ]);
    }

    table.printstd();

    Ok(())
}