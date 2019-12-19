use colored::Colorize;

use crate::{Result, System};

mod experiments;
mod controller;
mod runners;

pub async fn status(mut system: System) -> Result<()> {
    print_section(controller::print(&mut system).await, true);
    print_section(experiments::print(&mut system).await, true);
    print_section(runners::print(&mut system).await, false);

    Ok(())
}

fn print_section(result: Result<()>, add_newline: bool) {
    if let Err(err) = result {
        println!("{}: {}", "Couldn't fetch data".red(), err);
    }

    if add_newline {
        println!();
    }
}