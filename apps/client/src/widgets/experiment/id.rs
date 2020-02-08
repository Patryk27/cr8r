use std::fmt;

use colored::Colorize;

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
        write!(f, "{}", self.id.as_num().to_string().bright_cyan())
    }
}