use std::fmt;

use colored::Colorize;

use lib_interop::models::DRunnerName;

pub struct RunnerNameWidget<'a> {
    name: &'a DRunnerName,
}

impl<'a> RunnerNameWidget<'a> {
    pub fn new(name: &'a DRunnerName) -> Self {
        Self { name }
    }
}

impl fmt::Display for RunnerNameWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.as_str().bright_cyan())
    }
}