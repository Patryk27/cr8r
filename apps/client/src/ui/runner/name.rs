use std::fmt;

use lib_interop::protocol::core::PRunner;

pub struct RunnerName<'a> {
    runner: &'a PRunner,
}

impl<'a> RunnerName<'a> {
    pub fn new(runner: &'a PRunner) -> Self {
        Self { runner }
    }
}

impl fmt::Display for RunnerName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        write!(f, "{}", self.runner.name.bright_cyan())
    }
}