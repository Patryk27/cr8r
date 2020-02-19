use std::fmt;

use colored::Colorize;

use lib_interop::models::job::DJobStatus;

pub struct JobStatusWidget<'a> {
    status: &'a DJobStatus,
}

impl<'a> JobStatusWidget<'a> {
    pub fn new(status: &'a DJobStatus) -> Self {
        Self { status }
    }
}

impl fmt::Display for JobStatusWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DJobStatus::*;

        write!(f, "{}", match self.status {
            Pending => {
                "pending".yellow().to_string()
            }

            Running => {
                "running".green().to_string()
            }

            Completed { result } => {
                let state = "completed".blue().bold();

                let result = match result {
                    Ok(_) => {
                        "success".green().to_string()
                    }

                    Err(err) => {
                        format!("{}: {}", "failure".red(), err)
                    }
                };

                format!("{}\n.. {}", state, result)
            }
        })
    }
}