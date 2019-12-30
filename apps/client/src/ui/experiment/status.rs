use std::fmt;

use lib_interop::contract::CExperimentStatus;

pub struct ExperimentStatus<'a> {
    status: &'a CExperimentStatus,
}

impl<'a> ExperimentStatus<'a> {
    pub fn new(status: &'a CExperimentStatus) -> Self {
        Self { status }
    }
}

impl fmt::Display for ExperimentStatus<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::ui;
        use colored::Colorize;

        write!(f, "{}", match self.status {
            CExperimentStatus::Idle { since } => {
                let state = "idle / awaiting runner".yellow();
                let since = ui::DateTime::new(*since);

                format!("{} (since {})", state, since)
            }

            CExperimentStatus::Running { since, completed_jobs, total_jobs, .. } => {
                let state = "running".green();

                let completed_jobs = completed_jobs
                    .to_string()
                    .blue();

                let total_jobs = total_jobs
                    .to_string()
                    .blue();

                let since = ui::DateTime::new(*since);

                format!("{} (completed {} out of {} jobs(s), since {})", state, completed_jobs, total_jobs, since)
            }

            CExperimentStatus::Completed { since, result } => {
                let state = "completed"
                    .blue()
                    .bold();

                let result = if result.is_ok() {
                    "success"
                        .green()
                        .to_string()
                } else {
                    "failure"
                        .red()
                        .to_string()
                };

                let since = ui::DateTime::new(*since);

                format!("{} ({}, since {})", state, result, since)
            }

            CExperimentStatus::Zombie { since } => {
                let state = "zombie".red();
                let since = ui::DateTime::new(*since);

                format!("{} (since {})", state, since)
            }
        })
    }
}