use std::fmt;

use lib_protocol::core::PRunner;

pub struct RunnersTable<'a> {
    runners: &'a [PRunner],
}

impl<'a> RunnersTable<'a> {
    pub fn new(runners: &'a [PRunner]) -> Self {
        Self { runners }
    }
}

impl fmt::Display for RunnersTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::ui;
        use colored::*;
        use prettytable::*;

        if self.runners.is_empty() {
            return write!(f, "There are no runners");
        }

        let mut table = Table::new();

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        table.set_titles(row![
            "Name", "Status", "Joined at", "Last heartbeat at",
        ]);

        for runner in self.runners {
            let name = runner.name.bright_cyan();
            let status = ui::RunnerStatus::new(runner);
            let joined_at = ui::DateTime::new(&runner.joined_at);
            let last_heartbeat_at = ui::DateTime::new(&runner.last_heartbeat_at);

            table.add_row(row![
                name,
                status,
                joined_at,
                last_heartbeat_at,
            ]);
        }

        write!(f, "{}", table)
    }
}