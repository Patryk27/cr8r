use std::fmt;

use prettytable::{cell, row};

use lib_core_ui::*;
use lib_interop::models::DRunner;

use crate::widgets::{RunnerNameWidget, RunnerStatusWidget};

pub struct RunnerListWidget<'a> {
    runners: &'a [DRunner],
}

impl<'a> RunnerListWidget<'a> {
    pub fn new(runners: &'a [DRunner]) -> Self {
        Self { runners }
    }
}

impl fmt::Display for RunnerListWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.runners.is_empty() {
            return writeln!(f, "There are no runners");
        }

        let mut table = table! {
            titles: ["▲ Name", "Status", "Joined at", "Last heartbeat at"],
        };

        for runner in self.runners {
            table.add_row(row![
                RunnerNameWidget::new(&runner.name),
                RunnerStatusWidget::new(&runner.status),
                DateTimeWidget::new(runner.joined_at),
                DateTimeWidget::new(runner.last_heartbeat_at),
            ]);
        }

        write!(f, "{}", table)
    }
}