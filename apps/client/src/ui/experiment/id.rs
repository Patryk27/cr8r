use std::fmt;

use lib_interop::contract::CExperimentId;

pub struct ExperimentId<'a> {
    id: &'a CExperimentId,
}

impl<'a> ExperimentId<'a> {
    pub fn new(id: &'a CExperimentId) -> Self {
        Self { id }
    }
}

impl fmt::Display for ExperimentId<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        write!(f, "{}", self.id.as_str().bright_cyan())
    }
}