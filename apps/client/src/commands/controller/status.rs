use colored::Colorize;
use prettytable::*;

use crate::{Result, System};

mod experiments;
mod controller;
mod runners;

pub async fn run(mut system: System) -> Result<()> {
    print(controller::print(&mut system).await, true);
    print(experiments::print(&mut system).await, true);
    print(runners::print(&mut system).await, false);

    Ok(())
}

fn print(result: Result<()>, add_newline: bool) {
    if let Err(err) = result {
        println!("{}: {}", "Could not fetch data".red(), err);
    }

    if add_newline {
        println!();
    }
}

fn table(titles: Row) -> Table {
    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(titles);
    table
}