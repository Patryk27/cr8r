use colored::Colorize;
use prettytable::*;

use lib_protocol::core::p_experiment::p_status::*;
use lib_protocol::core::PExperiment;
use lib_protocol::for_client::PFindExperimentsRequest;

use crate::{Result, spinner, System};
use crate::ui::reformat_datetime;

use super::table;

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", "# Experiments".blue());
    println!();

    let experiments = spinner! {
        system
            .client().await?
            .find_experiments(PFindExperimentsRequest {}).await?
            .into_inner()
            .experiments
    };

    if experiments.is_empty() {
        println!("There are no experiments registered");
        return Ok(());
    }

    let mut table = table(row![
        "Id", "Status", "Created at",
    ]);

    for experiment in experiments {
        let id = experiment.id.bright_cyan();
        let status = status_to_string(&experiment).unwrap_or_else(|| "invalid status".to_string());
        let created_at = reformat_datetime(&experiment.created_at);

        table.add_row(row![
            id, status, created_at,
        ]);
    }

    table.printstd();

    Ok(())
}

fn status_to_string(experiment: &PExperiment) -> Option<String> {
    Some(match experiment.status.as_ref()?.op.as_ref()? {
        Op::AwaitingRunner(PAwaitingRunner { since }) => {
            let state = "awaiting runner".purple();
            let since = reformat_datetime(since);

            format!("{} (since {})", state, state)
        }

        Op::Running(PRunning { since, completed_scenarios, .. }) => {
            let state = "running".green();

            let completed = completed_scenarios
                .to_string()
                .blue();

            let all = experiment.scenario_count
                .to_string()
                .blue();

            let since = reformat_datetime(since);

            format!("{} (completed {} of {} scenario(s), since {})", state, completed, all, since)
        }

        Op::Completed(PCompleted { since, success }) => {
            let state = "completed".blue();

            let success = if *success {
                "success"
                    .green()
                    .to_string()
            } else {
                "failure"
                    .red()
                    .to_string()
            };

            let since = reformat_datetime(since);

            format!("{} ({}, since {})", state, success, since)
        }

        Op::Aborted(PAborted { since }) => {
            let state = "aborted".red();
            let since = reformat_datetime(since);

            format!("{} (since {})", state, since)
        }

        Op::Zombie(PZombie { since }) => {
            let state = "zombie".red();
            let since = reformat_datetime(since);

            format!("{} (since {})", state, since)
        }
    })
}