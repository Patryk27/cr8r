use std::convert::TryInto;

use anyhow::*;

use lib_core_ui::*;
use lib_interop::domain::DExperimentId;
use lib_interop::proto::models::PExperimentId;

use crate::modules::app::AppContext;
use crate::modules::attachment::AttachmentRepository;
use crate::modules::experiment::ExperimentRepository;
use crate::modules::job::JobRepository;
use crate::modules::report::ReportRepository;
use crate::widgets::{ExperimentDetailsWidget, ReportListWidget};

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

    let experiment = spinner! {
        ExperimentRepository::new(ctxt)
            .await?
            .find_one(id)
            .await?
            .ok_or_else(|| anyhow!("No such experiment exists"))?
    };

    if show_attachments || show_jobs || show_reports {
        HeaderWidget::new("Experiment")
            .println();
    }

    ExperimentDetailsWidget::new(&experiment)
        .println();

    if show_attachments {
        print_attachments(ctxt, id)
            .await?;
    }

    if show_jobs {
        print_jobs(ctxt, id)
            .await?;
    }

    if show_reports {
        print_reports(ctxt, id)
            .await?;
    }

    Ok(())
}

async fn print_attachments(ctxt: &mut AppContext, id: DExperimentId) -> Result<()> {
    HeaderWidget::new("Attachments")
        .println();

    let attachments = spinner! {
        AttachmentRepository::new(ctxt)
            .await?
            .find(id)
            .await?
    };

    println!("{:#?}", attachments); // @todo
    println!();

    Ok(())
}

async fn print_jobs(ctxt: &mut AppContext, id: DExperimentId) -> Result<()> {
    HeaderWidget::new("Jobs")
        .println();

    let jobs = spinner! {
        JobRepository::new(ctxt)
            .await?
            .find(id)
            .await?
    };

    println!("{:#?}", jobs); // @todo
    println!();

    Ok(())
}

async fn print_reports(ctxt: &mut AppContext, id: DExperimentId) -> Result<()> {
    HeaderWidget::new("Reports")
        .println();

    let reports = spinner! {
        ReportRepository::new(ctxt)
            .await?
            .find(id)
            .await?
    };

    ReportListWidget::new(&reports)
        .println();

    Ok(())
}