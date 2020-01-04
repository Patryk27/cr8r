#![feature(type_ascription)]

use anyhow::Error;
use colored::Colorize;

pub trait PrintableError {
    fn print(&self);
}

impl PrintableError for Error {
    fn print(&self) {
        eprintln!("Error:");
        eprintln!("  {}", format!("{}", self).red());

        let causes = self
            .chain()
            .skip(1);

        for cause in causes {
            eprintln!();
            eprintln!("Caused by:");
            eprintln!("  {}", format!("{}", cause).yellow());
        }
    }
}