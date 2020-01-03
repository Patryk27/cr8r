use std::fmt;

use lib_interop::domain::DExperiment;

pub struct ExperimentsTable<'a> {
    experiments: &'a [DExperiment],
}

impl<'a> ExperimentsTable<'a> {
    pub fn new(experiments: &'a [DExperiment]) -> Self {
        Self { experiments }
    }
}

impl fmt::Display for ExperimentsTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::{table, ui};
        use prettytable::{cell, row};

        if self.experiments.is_empty() {
            return writeln!(f, "There are no experiments");
        }

        let mut table = table! {
            titles: ["Id", "Status", "▼ Created at"],
        };

        for experiment in self.experiments {
            table.add_row(row![
                ui::ExperimentId::new(&experiment.id),
                ui::ExperimentStatus::new(&experiment.status),
                ui::DateTime::new(experiment.created_at),
            ]);
        }

        write!(f, "{}", table)
    }
}