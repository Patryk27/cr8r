use std::fmt;

use lib_interop::domain::DRunner;
use lib_ui::table;

pub struct RunnersTable<'a> {
    runners: &'a [DRunner],
}

impl<'a> RunnersTable<'a> {
    pub fn new(runners: &'a [DRunner]) -> Self {
        Self { runners }
    }
}

impl fmt::Display for RunnersTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::ui;
        use prettytable::{cell, row};

        if self.runners.is_empty() {
            return writeln!(f, "There are no runners");
        }

        let mut table = table! {
            titles: ["▲ Name", "Status", "Joined at", "Last heartbeat at"],
        };

        for runner in self.runners {
            table.add_row(row![
                ui::RunnerName::new(&runner.name),
                ui::RunnerStatus::new(&runner.status),
                lib_ui::DateTime::new(runner.joined_at),
                lib_ui::DateTime::new(runner.last_heartbeat_at),
            ]);
        }

        write!(f, "{}", table)
    }
}