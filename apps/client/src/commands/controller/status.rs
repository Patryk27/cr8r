use colored::Colorize;
use prettytable::*;

use crate::{Result, System};

pub async fn run(system: System) -> Result<()> {
//    let status = system
//        .client()
//        .controller_status()?;
//
//    print_prelude(system.config());
//    println!();
//    print_experiments(status.experiments);
//    println!();
//    print_runners(status.runners);

    Ok(())
}

//fn print_prelude(config: &Config) {
//    println!("Status of controller `{}`:", config.controller.address.green());
//}
//
//fn print_experiments(experiments: Vec<Experiment>) {
//    println!("{}", "# Experiments".blue());
//    println!();
//
//    if experiments.is_empty() {
//        println!("There are no experiments registered.");
//        return;
//    }
//
//    let mut table = table(row![
//        "Id", "Definition", "Status",
//    ]);
//
//    for experiment in experiments {
//        let definition = format!("{:?}", experiment.definition);
//
//        let status = match experiment.status {
//            ExperimentStatus::AwaitingRunner => {
//                "awaiting runner"
//                    .purple()
//                    .to_string()
//            }
//
//            ExperimentStatus::Finished => {
//                "finished"
//                    .green()
//                    .to_string()
//            }
//
//            ExperimentStatus::Running { runner_id } => {
//                format!("{} (on runner `{}`)", "running".blue(), runner_id)
//            }
//        };
//
//        table.add_row(row![
//            experiment.id,
//            definition,
//            status,
//        ]);
//    }
//
//    table.printstd();
//}
//
//fn print_runners(runners: Vec<Runner>) {
//    println!("{}", "# Runners".blue());
//    println!();
//
//    if runners.is_empty() {
//        println!("There are no runners registered.");
//        return;
//    }
//
//    let mut table = table(row![
//        "Id", "Name", "Status",
//    ]);
//
//    for runner in runners {
//        let status = match runner.status {
//            RunnerStatus::Idle => {
//                "idle"
//                    .green()
//                    .to_string()
//            }
//
//            RunnerStatus::Initializing => {
//                "initializing"
//                    .purple()
//                    .to_string()
//            }
//
//            RunnerStatus::Working { experiment_id } => {
//                format!("{} (on experiment `{}`)", "working".blue(), experiment_id)
//            }
//        };
//
//        table.add_row(row![
//            runner.id,
//            runner.name,
//            status,
//        ]);
//    }
//
//    table.printstd();
//}
//
//fn table(titles: Row) -> Table {
//    let mut table = Table::new();
//
//    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
//    table.set_titles(titles);
//    table
//}