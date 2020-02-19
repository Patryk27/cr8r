use std::fmt;

use prettytable::{cell, row};

use lib_core_ui::*;
use lib_interop::models::DJob;

use crate::widgets::JobStatusWidget;

pub struct JobListWidget<'a> {
    jobs: &'a [DJob],
}

impl<'a> JobListWidget<'a> {
    pub fn new(jobs: &'a [DJob]) -> Self {
        Self { jobs }
    }
}

impl fmt::Display for JobListWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.jobs.is_empty() {
            return writeln!(f, "There are no jobs");
        }

        let mut table = table! {
            format: FORMAT_NO_BORDER,
            titles: ["Id", "Name", "Status"],
        };

        for job in self.jobs {
            let status = JobStatusWidget::new(&job.status);

            table.add_row(row![
                job.id,
                job.name,
                status,
            ]);
        }

        write!(f, "{}", table)
    }
}