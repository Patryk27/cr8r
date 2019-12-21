use std::fmt;

use lib_interop::protocol::core::PExperiment;

pub struct ExperimentId<'a> {
    experiment: &'a PExperiment,
}

impl<'a> ExperimentId<'a> {
    pub fn new(experiment: &'a PExperiment) -> Self {
        Self { experiment }
    }
}

impl fmt::Display for ExperimentId<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        write!(f, "{}", self.experiment.id.bright_cyan())
    }
}