use std::fmt;

use lib_interop::domain::DRunnerStatus;

pub struct RunnerStatus<'a> {
    status: &'a DRunnerStatus,
}

impl<'a> RunnerStatus<'a> {
    pub fn new(status: &'a DRunnerStatus) -> Self {
        Self { status }
    }
}

impl fmt::Display for RunnerStatus<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        write!(f, "{}", match self.status {
            DRunnerStatus::Idle { since } => {
                let status = "idle / awaiting experiment".yellow();
                let since = lib_ui::DateTime::new(*since);

                format!("{} (since {})", status, since)
            }

            DRunnerStatus::Working { since, .. } => {
                let status = "working".green();
                let since = lib_ui::DateTime::new(*since);

                format!("{} (since {})", status, since)
            }

            DRunnerStatus::Zombie { since } => {
                let status = "zombie".red();
                let since = lib_ui::DateTime::new(*since);

                format!("{} (since {})", status, since)
            }
        })
    }
}