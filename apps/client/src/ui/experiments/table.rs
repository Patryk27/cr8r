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
        use crate::ui;
        use colored::*;
        use prettytable::*;

        if self.experiments.is_empty() {
            return write!(f, "There are no experiments");
        }

        let mut table = Table::new();

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        table.set_titles(row![
            "Id", "Status", "Created at",
        ]);

        for experiment in self.experiments {
            let id = experiment.id.bright_cyan();
            let status = ui::ExperimentStatus::new(experiment);
            let created_at = ui::DateTime::new(&experiment.created_at);

            table.add_row(row![
                id,
                status,
                created_at,
            ]);
        }

        write!(f, "{}", table)
    }
}