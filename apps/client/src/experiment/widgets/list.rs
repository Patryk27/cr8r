use std::fmt;

use prettytable::{cell, row};

use lib_interop::domain::DExperiment;
use lib_ui::*;

use crate::experiment::{ExperimentIdWidget, ExperimentStatusWidget};

pub struct ExperimentListWidget<'a> {
    experiments: &'a [DExperiment],
}

impl<'a> ExperimentListWidget<'a> {
    pub fn new(experiments: &'a [DExperiment]) -> Self {
        Self { experiments }
    }
}

impl fmt::Display for ExperimentListWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.experiments.is_empty() {
            return writeln!(f, "There are no experiments");
        }

        let mut table = table! {
            titles: ["Id", "Status", "▼ Created at"],
        };

        for experiment in self.experiments {
            table.add_row(row![
                ExperimentIdWidget::new(&experiment.id),
                ExperimentStatusWidget::new(&experiment.status),
                DateTimeWidget::new(experiment.created_at),
            ]);
        }

        write!(f, "{}", table)
    }
}