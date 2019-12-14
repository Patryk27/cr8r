use colored::Colorize;
use prettytable::*;

use lib_protocol::core::p_runner::p_status::*;
use lib_protocol::core::PRunner;
use lib_protocol::for_client::PFindRunnersRequest;

use crate::{Result, spinner, System};
use crate::ui::reformat_datetime;

use super::table;

pub async fn print(system: &mut System) -> Result<()> {
    println!("{}", "# Runners".blue());
    println!();

    let runners = spinner! {
        system
            .client().await?
            .find_runners(PFindRunnersRequest {}).await?
            .into_inner()
            .runners
    };

    if runners.is_empty() {
        println!("There are no runners registered");
        return Ok(());
    }

    let mut table = table(row![
        "Name", "Status", "Joined at", "Last heartbeat at",
    ]);

    for runner in runners {
        let name = runner.name.bright_cyan();
        let status = status_to_string(&runner).unwrap_or_else(|| "invalid status".to_string());
        let joined_at = reformat_datetime(&runner.joined_at);
        let last_heartbeat_at = reformat_datetime(&runner.last_heartbeat_at);

        table.add_row(row![
            name, status, joined_at, last_heartbeat_at,
        ]);
    }

    table.printstd();

    Ok(())
}

fn status_to_string(runner: &PRunner) -> Option<String> {
    Some(match runner.status.as_ref()?.op.as_ref()? {
        Op::Idle(PIdle { since }) => {
            let status = "idle".purple();
            let since = reformat_datetime(since);

            format!("{} (since {})", status, since)
        }

        Op::Working(PWorking { since, .. }) => {
            let status = "working".green();
            let since = reformat_datetime(&since);

            format!("{} (since {})", status, since)
        }

        Op::Zombie(PZombie { since }) => {
            let status = "zombie".red();
            let since = reformat_datetime(&since);

            format!("{} (since {})", status, since)
        }
    })
}