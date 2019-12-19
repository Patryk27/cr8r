use std::fmt;

use lib_protocol::core::PExperimentStep;

pub struct ExperimentStepsTable<'a> {
    steps: &'a [PExperimentStep],
}

impl<'a> ExperimentStepsTable<'a> {
    pub fn new(steps: &'a [PExperimentStep]) -> Self {
        Self { steps }
    }
}

impl fmt::Display for ExperimentStepsTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::{table, ui};
        use prettytable::{cell, row};

        let mut table = table! {
            titles: ["Id", "Step"],
        };

        for (step_id, step) in self.steps.iter().enumerate() {
            table.add_row(row![
                step_id,
                ui::InlineExperimentStep::new(step),
            ]);
        }

        write!(f, "{}", table)
    }
}