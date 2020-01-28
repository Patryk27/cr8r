use anyhow::*;

use lib_interop::convert;
use lib_interop::domain::DExperiment;
use lib_interop::proto::controller::{PFindExperimentsRequest, PFindReportsRequest};
use lib_ui::*;

use crate::app::AppContext;
use crate::experiment::ExperimentDetailsWidget;
use crate::report::ReportListWidget;

pub async fn show(
    ctxt: &mut AppContext,
    id: &str,
    show_jobs: bool,
    show_reports: bool,
) -> Result<()> {
    let mut experiments = spinner! {
        ctxt.client()
            .await?
            .find_experiments(PFindExperimentsRequest { id: id.into() })
            .await?
            .experiments
    };

    if let Some(experiment) = experiments.drain(..).next() {
        let experiment = convert!(experiment as _?);

        if show_jobs || show_reports {
            println!("{}", HeaderWidget::new("Experiment"));
        }

        print_experiment(&experiment);

        if show_jobs {
//            unimplemented!() @todo
        }

        if show_reports {
            if let Err(err) = print_reports(ctxt, &id).await {
                // @todo this should be a proper error message
                println!("Couldn't print reports: {}", err);
            }
        }
    } else {
        println!("No such experiment exists");
    }

    Ok(())
}

fn print_experiment(experiment: &DExperiment) {
    println!("{}", ExperimentDetailsWidget::new(experiment));
}

async fn print_reports(ctxt: &mut AppContext, id: &str) -> Result<()> {
    println!("{}", HeaderWidget::new("Reports"));

    let reports = spinner! {
        ctxt.client()
            .await?
            .find_reports(PFindReportsRequest { experiment_id: id.into() })
            .await?
            .reports
    };

    let reports = convert!(reports as [_?]);

    println!("{}", ReportListWidget::new(&reports));

    Ok(())
}