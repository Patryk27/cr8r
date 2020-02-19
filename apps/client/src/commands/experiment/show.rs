use std::convert::TryInto;

use anyhow::*;

use lib_core_ui::*;
use lib_interop::connection::Connection;
use lib_interop::models::DExperimentId;
use lib_interop::proto::models::PExperimentId;

use crate::modules::app::AppContext;
use crate::widgets::{ExperimentDetailsWidget, JobListWidget, ReportListWidget};

pub async fn show(
    ctxt: &mut AppContext,
    id: PExperimentId,
    show_attachments: bool,
    show_jobs: bool,
    show_reports: bool,
) -> Result<()> {
    let id = id
        .try_into()
        .context("Given experiment id is not valid")?;

    let conn = ctxt.conn().await?;

    let experiment = spinner! {
        conn.experiments()
            .find_one(id).await?
    };

    if show_attachments || show_jobs || show_reports {
        HeaderWidget::new("Experiment")
            .println();
    }

    ExperimentDetailsWidget::new(&experiment)
        .println();

    if show_attachments {
        print_attachments(conn.clone(), id).await?;
    }

    if show_jobs {
        print_jobs(conn.clone(), id).await?;
    }

    if show_reports {
        print_reports(conn.clone(), id).await?;
    }

    Ok(())
}

async fn print_attachments(conn: Connection, id: DExperimentId) -> Result<()> {
    HeaderWidget::new("Attachments")
        .println();

    let attachments = spinner! {
        conn.attachments()
            .find_many(id).await?
    };

    println!("{:#?}", attachments); // @todo
    println!();

    Ok(())
}

async fn print_jobs(conn: Connection, id: DExperimentId) -> Result<()> {
    HeaderWidget::new("Jobs")
        .println();

    let jobs = spinner! {
        conn.jobs()
            .find_many(id).await?
    };

    JobListWidget::new(&jobs)
        .println();

    Ok(())
}

async fn print_reports(conn: Connection, id: DExperimentId) -> Result<()> {
    HeaderWidget::new("Reports")
        .println();

    let reports = spinner! {
        conn.reports()
            .find_many(id).await?
    };

    ReportListWidget::new(&reports)
        .println();

    Ok(())
}