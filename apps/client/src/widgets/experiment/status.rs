use std::fmt;

use colored::Colorize;

use lib_core_ui::*;
use lib_interop::models::DExperimentStatus;

pub struct ExperimentStatusWidget<'a> {
    status: &'a DExperimentStatus,
}

impl<'a> ExperimentStatusWidget<'a> {
    pub fn new(status: &'a DExperimentStatus) -> Self {
        Self { status }
    }
}

impl fmt::Display for ExperimentStatusWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self.status {
            DExperimentStatus::Idle { since } => {
                let state = "awaiting runner".yellow();
                let since = DateTimeWidget::new(*since);

                format!("{}\n.. since {}", state, since)
            }

            DExperimentStatus::Running { since, completed_jobs, total_jobs, .. } => {
                let state = "running".green();
                let completed_jobs = completed_jobs.to_string().blue();
                let total_jobs = total_jobs.to_string().blue();
                let since = DateTimeWidget::new(*since);

                format!("{}\n.. completed {} out of {} jobs(s)\n.. since {}", state, completed_jobs, total_jobs, since)
            }

            DExperimentStatus::Completed { since, result } => {
                let state = "completed".blue().bold();

                let result = if result.is_ok() {
                    "success".green().to_string()
                } else {
                    "failure".red().to_string()
                };

                let since = DateTimeWidget::new(*since);

                format!("{} ({})\n.. since {}", state, result, since)
            }

            DExperimentStatus::Stopped { since } => {
                let state = "stopped".white().dimmed();
                let since = DateTimeWidget::new(*since);

                format!("{}\n.. since {}", state, since)
            }
        })
    }
}