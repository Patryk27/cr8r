use std::fmt;

use lib_interop::contract::CExperiment;

pub struct ExperimentDetails<'a> {
    experiment: &'a CExperiment,
}

impl<'a> ExperimentDetails<'a> {
    pub fn new(experiment: &'a CExperiment) -> Self {
        Self { experiment }
    }
}

impl fmt::Display for ExperimentDetails<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::ui;
        use prettytable::*;

        let mut table = Table::new();

        table.add_row(row![
            "Id", ui::ExperimentId::new(&self.experiment.id),
        ]);

        table.add_row(row![
            "Created at", ui::DateTime::new(self.experiment.created_at),
        ]);

        table.add_row(row![
            "Status", ui::ExperimentStatus::new(&self.experiment.status),
        ]);

        write!(f, "{}", table)
    }
}