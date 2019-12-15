use std::fmt;

use colored::Colorize;

use lib_protocol::core::p_experiment::p_status::*;
use lib_protocol::core::PExperiment;

use crate::ui;

pub struct ExperimentStatus<'a> {
    experiment: &'a PExperiment,
}

impl<'a> ExperimentStatus<'a> {
    pub fn new(experiment: &'a PExperiment) -> Self {
        Self { experiment }
    }
}

impl fmt::Display for ExperimentStatus<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = try {
            match self.experiment.status.as_ref()?.op.as_ref()? {
                Op::AwaitingRunner(PAwaitingRunner { since }) => {
                    let state = "awaiting runner".yellow();
                    let since = ui::DateTime::new(since);

                    format!("{} (since {})", state, since)
                }

                Op::Running(PRunning { since, completed_scenarios, .. }) => {
                    let state = "running".green();

                    let completed = completed_scenarios
                        .to_string()
                        .blue();

                    let all = self.experiment.scenario_count
                        .to_string()
                        .blue();

                    let since = ui::DateTime::new(since);

                    format!("{} (completed {} of {} scenario(s), since {})", state, completed, all, since)
                }

                Op::Completed(PCompleted { since, success }) => {
                    let state = "completed"
                        .blue()
                        .bold();

                    let success = if *success {
                        "success"
                            .green()
                            .to_string()
                    } else {
                        "failure"
                            .red()
                            .to_string()
                    };

                    let since = ui::DateTime::new(since);

                    format!("{} ({}, since {})", state, success, since)
                }

                Op::Aborted(PAborted { since }) => {
                    let state = "aborted".red();
                    let since = ui::DateTime::new(since);

                    format!("{} (since {})", state, since)
                }

                Op::Zombie(PZombie { since }) => {
                    let state = "zombie".red();
                    let since = ui::DateTime::new(since);

                    format!("{} (since {})", state, since)
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