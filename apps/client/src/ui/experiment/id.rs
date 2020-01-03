use std::fmt;

use lib_interop::domain::DExperimentId;

pub struct ExperimentId<'a> {
    id: &'a DExperimentId,
}

impl<'a> ExperimentId<'a> {
    pub fn new(id: &'a DExperimentId) -> Self {
        Self { id }
    }
}

impl fmt::Display for ExperimentId<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        write!(f, "{}", self.id.as_str().bright_cyan())
    }
}