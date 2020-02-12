use std::convert::TryInto;

use anyhow::*;

use lib_core_ui::*;
use lib_interop::domain::DExperimentId;
use lib_interop::proto::core::PExperimentId;

use crate::modules::app::AppContext;
use crate::modules::experiment::ExperimentSearcher;
use crate::modules::report::ReportSearcher;
use crate::widgets::{ExperimentDetailsWidget, ReportListWidget};

pub async fn show(
    ctxt: &mut AppContext,
    id: PExperimentId,
    show_jobs: bool,
    show_reports: bool,
) -> Result<()> {
    let id = id
        .try_into()
        .context("Given experiment id is not valid")?;

    let experiment = spinner! {
        ExperimentSearcher::new(ctxt)
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("No such experiment exists"))?
    };

    if show_jobs || show_reports {
        HeaderWidget::new("Experiment")
            .println();
    }

    ExperimentDetailsWidget::new(&experiment)
        .println();

    if show_jobs {
        do_show_jobs(ctxt, id)
            .await?;
    }

    if show_reports {
        do_show_reports(ctxt, id)
            .await?;
    }

    Ok(())
}

async fn do_show_jobs(ctxt: &mut AppContext, id: DExperimentId) -> Result<()> {
    HeaderWidget::new("Jobs")
        .println();

    // @todo

    Ok(())
}

async fn do_show_reports(ctxt: &mut AppContext, id: DExperimentId) -> Result<()> {
    HeaderWidget::new("Reports")
        .println();

    let reports = spinner! {
        ReportSearcher::new(ctxt)
            .find_by_experiment_id(id)
            .await?
    };

    ReportListWidget::new(&reports)
        .println();

    Ok(())
}