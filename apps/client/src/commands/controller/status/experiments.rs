use colored::Colorize;
use prettytable::*;

use lib_protocol::core::p_experiment::p_status::*;
use lib_protocol::for_client::PFindExperimentsRequest;

use crate::{Result, System};

use super::table;

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", "# Experiments".blue());
    println!();

    let experiments = system
        .client().await?
        .find_experiments(PFindExperimentsRequest {}).await?
        .into_inner()
        .experiments;

    if experiments.is_empty() {
        println!("There are no experiments registered");
        return Ok(());
    }

    let mut table = table(row![
        "Id", "Status", "Created at", "Heartbeaten at",
    ]);

    for experiment in experiments {
        let status = match experiment.status.unwrap().op.unwrap() {
            Op::AwaitingRunner(PAwaitingRunner { since }) => {
                format!(
                    "{} (since {})",
                    "awaiting runner".purple(),
                    since,
                )
            }

            Op::Running(PRunning { since, completed_scenarios, total_scenarios, .. }) => {
                format!(
                    "{} (completed {} of {} scenario(s), since {})",
                    "running".green(),
                    completed_scenarios,
                    total_scenarios,
                    since,
                )
            }

            Op::Completed(PCompleted { since }) => {
                format!(
                    "{} (since {})",
                    "completed".blue(),
                    since,
                )
            }

            Op::Aborted(PAborted { since }) => {
                format!(
                    "{} (since {})",
                    "aborted".red(),
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
            experiment.id.bright_cyan(),
            status,
            experiment.created_at,
            experiment.heartbeaten_at,
        ]);
    }

    table.printstd();

    Ok(())
}