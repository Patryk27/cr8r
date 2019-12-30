use std::fmt;

use lib_interop::contract::CRunnerStatus;

pub struct RunnerStatus<'a> {
    status: &'a CRunnerStatus,
}

impl<'a> RunnerStatus<'a> {
    pub fn new(status: &'a CRunnerStatus) -> Self {
        Self { status }
    }
}

impl fmt::Display for RunnerStatus<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::ui;
        use colored::Colorize;

        write!(f, "{}", match self.status {
            CRunnerStatus::Idle { since } => {
                let status = "idle / awaiting experiment".yellow();
                let since = ui::DateTime::new(*since);

                format!("{} (since {})", status, since)
            }

            CRunnerStatus::Working { since, .. } => {
                let status = "working".green();
                let since = ui::DateTime::new(*since);

                format!("{} (since {})", status, since)
            }

            CRunnerStatus::Zombie { since } => {
                let status = "zombie".red();
                let since = ui::DateTime::new(*since);

                format!("{} (since {})", status, since)
            }
        })
    }
}