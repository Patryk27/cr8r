use std::fmt;

use colored::Colorize;

use lib_core_ui::*;
use lib_interop::models::DRunnerStatus;

pub struct RunnerStatusWidget<'a> {
    status: &'a DRunnerStatus,
}

impl<'a> RunnerStatusWidget<'a> {
    pub fn new(status: &'a DRunnerStatus) -> Self {
        Self { status }
    }
}

impl fmt::Display for RunnerStatusWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self.status {
            DRunnerStatus::Idle { since } => {
                let status = "idle / awaiting experiment".yellow();
                let since = DateTimeWidget::new(*since);

                format!("{} (since {})", status, since)
            }

            DRunnerStatus::Working { since, .. } => {
                let status = "working".green();
                let since = DateTimeWidget::new(*since);

                format!("{} (since {})", status, since)
            }

            DRunnerStatus::Zombie { since } => {
                let status = "zombie".red();
                let since = DateTimeWidget::new(*since);

                format!("{} (since {})", status, since)
            }
        })
    }
}