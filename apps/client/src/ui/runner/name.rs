use std::fmt;

use lib_interop::domain::DRunnerName;

pub struct RunnerName<'a> {
    name: &'a DRunnerName,
}

impl<'a> RunnerName<'a> {
    pub fn new(name: &'a DRunnerName) -> Self {
        Self { name }
    }
}

impl fmt::Display for RunnerName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        write!(f, "{}", self.name.as_str().bright_cyan())
    }
}