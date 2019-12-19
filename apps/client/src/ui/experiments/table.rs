use std::fmt;

use lib_protocol::core::PExperiment;

pub struct ExperimentsTable<'a> {
    experiments: &'a [PExperiment],
}

impl<'a> ExperimentsTable<'a> {
    pub fn new(experiments: &'a [PExperiment]) -> Self {
        Self { experiments }
    }
}

impl fmt::Display for ExperimentsTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::{table, ui};
        use prettytable::{cell, row};

        if self.experiments.is_empty() {
            return write!(f, "There are no experiments");
        }

        let mut table = table! {
            titles: ["Id", "Status", "Created at"],
        };

        for experiment in self.experiments {
            table.add_row(row![
                ui::ExperimentId::new(experiment),
                ui::ExperimentStatus::new(experiment),
                ui::DateTime::new(&experiment.created_at),
            ]);
        }

        write!(f, "{}", table)
    }
}