use std::fmt;

use lib_interop::protocol::core::PRunner;

pub struct RunnerStatus<'a> {
    runner: &'a PRunner,
}

impl<'a> RunnerStatus<'a> {
    pub fn new(runner: &'a PRunner) -> Self {
        Self { runner }
    }
}

impl fmt::Display for RunnerStatus<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::ui;
        use colored::Colorize;
        use lib_interop::protocol::core::p_runner::p_status::*;

        let status = try {
            match self.runner.status.as_ref()?.op.as_ref()? {
                Op::Idle(PIdle { since }) => {
                    let status = "idle / awaiting experiment".yellow();
                    let since = ui::DateTime::new(since);

                    format!("{} (since {})", status, since)
                }

                Op::Working(PWorking { since, .. }) => {
                    let status = "working".green();
                    let since = ui::DateTime::new(since);

                    format!("{} (since {})", status, since)
                }

                Op::Zombie(PZombie { since }) => {
                    let status = "zombie".red();
                    let since = ui::DateTime::new(since);

                    format!("{} (since {})", status, since)
                }
            }
        };

        match status {
            Some(status) => {
                write!(f, "{}", status)
            }

            None => {
                write!(f, "invalid status")
            }
        }
    }
}