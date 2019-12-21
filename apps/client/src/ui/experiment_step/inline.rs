use std::fmt;

use lib_interop::protocol::core::PExperimentStep;

pub struct InlineExperimentStep<'a> {
    step: &'a PExperimentStep,
}

impl<'a> InlineExperimentStep<'a> {
    pub fn new(step: &'a PExperimentStep) -> Self {
        Self { step }
    }
}

impl fmt::Display for InlineExperimentStep<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;
        use lib_interop::protocol::core::p_experiment_step::{Op::*, *};

        let str = self.step.op.as_ref().map(|op| match op {
            Exec(PExec { cmd }) => {
                format!("{} `{}`", "exec".blue(), cmd.yellow())
            }

            LogSystemMsg(PLogSystemMsg { msg }) => {
                format!("{} `{}`", "log-system-msg".blue(), msg.yellow())
            }

            LogUserMsg(PLogUserMsg { msg }) => {
                format!("{} `{}`", "log-user-msg".blue(), msg.yellow())
            }
        });

        write!(f, "{}", str.unwrap_or_default())
    }
}