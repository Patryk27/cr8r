use std::fmt;

use lib_interop::protocol::core::PExperiment;

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
        use crate::ui;
        use colored::Colorize;
        use lib_interop::protocol::core::p_experiment::p_status::*;

        let status = try {
            match self.experiment.status.as_ref()?.op.as_ref()? {
                Op::Idle(PIdle { since }) => {
                    let state = "idle / awaiting runner".yellow();
                    let since = ui::DateTime::new(since);

                    format!("{} (since {})", state, since)
                }

                Op::Running(PRunning { since, completed_ops, .. }) => {
                    let state = "running".green();

                    let completed_ops = completed_ops
                        .to_string()
                        .blue();

                    let all_steps = 0; // @todo

//                    let all_steps = self.experiment.steps
//                        .len()
//                        .to_string()
//                        .blue();

                    let since = ui::DateTime::new(since);

                    format!("{} (completed {} of {} step(s), since {})", state, completed_ops, all_steps, since)
                }

                Op::Completed(PCompleted { since, success, .. }) => {
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