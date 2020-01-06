use std::fmt;

use lib_interop::domain::DExperimentStatus;

pub struct ExperimentStatus<'a> {
    status: &'a DExperimentStatus,
}

impl<'a> ExperimentStatus<'a> {
    pub fn new(status: &'a DExperimentStatus) -> Self {
        Self { status }
    }
}

impl fmt::Display for ExperimentStatus<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        write!(f, "{}", match self.status {
            DExperimentStatus::Idle { since } => {
                let state = "idle / awaiting runner".yellow();
                let since = lib_ui::DateTime::new(*since);

                format!("{} (since {})", state, since)
            }

            DExperimentStatus::Running { since, completed_jobs, total_jobs, .. } => {
                let state = "running".green();

                let completed_jobs = completed_jobs
                    .to_string()
                    .blue();

                let total_jobs = total_jobs
                    .to_string()
                    .blue();

                let since = lib_ui::DateTime::new(*since);

                format!("{} (completed {} out of {} jobs(s), since {})", state, completed_jobs, total_jobs, since)
            }

            DExperimentStatus::Completed { since, result } => {
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

                let since = lib_ui::DateTime::new(*since);

                format!("{} ({}, since {})", state, result, since)
            }

            DExperimentStatus::Zombie { since } => {
                let state = "zombie".red();
                let since = lib_ui::DateTime::new(*since);

                format!("{} (since {})", state, since)
            }
        })
    }
}