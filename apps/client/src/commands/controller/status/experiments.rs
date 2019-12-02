use colored::Colorize;
use prettytable::*;

use lib_protocol::client::FindExperimentsRequest;
use lib_protocol::core::experiment::status;

use crate::{Result, System};

use super::table;

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", "# Experiments".blue());
    println!();

    let experiments = system
        .client().await?
        .find_experiments(FindExperimentsRequest {}).await?
        .into_inner()
        .experiments;

    if experiments.is_empty() {
        println!("There are no experiments registered.");
        return Ok(());
    }

    let mut table = table(row![
        "Id", "Status", "Created at", "Heartbeaten at",
    ]);

    for experiment in experiments {
        let status = match experiment.status.unwrap().op.unwrap() {
            status::Op::AwaitingRunner(status::AwaitingRunner { since }) => {
                format!(
                    "{} (since {})",
                    "awaiting runner".purple(),
                    since,
                )
            }

            status::Op::Completed(status::Completed { since }) => {
                format!(
                    "{} (since {})",
                    "completed".blue(),
                    since,
                )
            }

            status::Op::Running(status::Running { since, completed_scenarios, total_scenarios, .. }) => {
                format!(
                    "{} (completed {} of {} scenario(s), since {})",
                    "running".green(),
                    completed_scenarios,
                    total_scenarios,
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
            experiment.id.bright_cyan(),
            status,
            experiment.created_at,
            experiment.heartbeaten_at,
        ]);
    }

    table.printstd();

    Ok(())
}