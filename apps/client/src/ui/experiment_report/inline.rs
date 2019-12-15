use std::fmt;

use colored::Colorize;

use lib_protocol::core::p_experiment_report::*;
use lib_protocol::core::PExperimentReport;

use crate::ui;

pub struct InlineExperimentReport<'a> {
    report: &'a PExperimentReport,
}

impl<'a> InlineExperimentReport<'a> {
    pub fn new(report: &'a PExperimentReport) -> Self {
        Self { report }
    }
}

impl fmt::Display for InlineExperimentReport<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = Kind::from_i32(self.report.kind)
            .unwrap_or(Kind::CustomMessage);

        write!(
            f,
            "{} {} | {}",
            format_created_at(&self.report.created_at),
            format_kind(kind),
            format_message(kind, &self.report.message),
        )
    }
}

fn format_created_at(datetime: &str) -> impl fmt::Display {
    ui::DateTime::new(datetime)
        .to_string()
        .dimmed()
}

fn format_kind(kind: Kind) -> impl fmt::Display {
    //@formatter:off
    let kind = match kind {
        Kind::SystemMessage => "sys ",
        Kind::CustomMessage => "msg ",
        Kind::ProcessOutput => "proc",
    };
    //@formatter:on

    kind.dimmed()
}

fn format_message(kind: Kind, message: &str) -> String {
    match kind {
        Kind::SystemMessage => message
            .blue()
            .to_string(),

        Kind::CustomMessage => message
            .white()
            .to_string(),

        Kind::ProcessOutput => message
            .white()
            .dimmed()
            .to_string(),
    }
}