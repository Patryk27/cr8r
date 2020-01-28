use std::fmt;

use lib_interop::domain::DExperimentId;

pub struct ExperimentIdWidget<'a> {
    id: &'a DExperimentId,
}

impl<'a> ExperimentIdWidget<'a> {
    pub fn new(id: &'a DExperimentId) -> Self {
        Self { id }
    }
}

impl fmt::Display for ExperimentIdWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        write!(f, "{}", self.id.as_str().bright_cyan())
    }
}