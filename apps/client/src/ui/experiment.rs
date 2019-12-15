use std::fmt;

use lib_protocol::core::PExperiment;

pub use self::status::*;

mod status;

pub struct Experiment<'a> {
    experiment: &'a PExperiment,
}

impl<'a> Experiment<'a> {
    pub fn new(experiment: &'a PExperiment) -> Self {
        Self { experiment }
    }
}

impl fmt::Display for Experiment<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::ui;
        use prettytable::*;

        let mut table = Table::new();

        table.add_row(row![
            "Id", self.experiment.id,
        ]);

        table.add_row(row![
            "Created at", ui::DateTime::new(&self.experiment.created_at),
        ]);

        table.add_row(row![
            "Number of scenarios", self.experiment.scenario_count,
        ]);

        table.add_row(row![
            "Status", ui::ExperimentStatus::new(self.experiment),
        ]);

        write!(f, "{}", table)
    }
}