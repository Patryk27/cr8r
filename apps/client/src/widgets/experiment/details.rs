use std::fmt;

use prettytable::*;

use lib_core_ui::*;
use lib_interop::domain::DExperiment;

use crate::widgets::{ExperimentIdWidget, ExperimentStatusWidget};

pub struct ExperimentDetailsWidget<'a> {
    experiment: &'a DExperiment,
}

impl<'a> ExperimentDetailsWidget<'a> {
    pub fn new(experiment: &'a DExperiment) -> Self {
        Self { experiment }
    }
}

impl fmt::Display for ExperimentDetailsWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();

        table.add_row(row![
            "Id",
            ExperimentIdWidget::new(&self.experiment.id),
        ]);

        table.add_row(row![
            "Created at",
            DateTimeWidget::new(self.experiment.created_at),
        ]);

        table.add_row(row![
            "Status",
            ExperimentStatusWidget::new(&self.experiment.status),
        ]);

        write!(f, "{}", table)
    }
}