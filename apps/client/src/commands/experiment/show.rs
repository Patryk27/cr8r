use anyhow::Result;

use lib_interop::convert;
use lib_interop::domain::DExperiment;
use lib_interop::proto::controller::{PFindExperimentsRequest, PFindReportsRequest};

use crate::{spinner, System, ui};

pub async fn show(
    mut system: System,
    id: &str,
    show_jobs: bool,
    show_reports: bool,
) -> Result<()> {
    let mut experiments = spinner! {
        system
            .client()
            .await?
            .find_experiments(PFindExperimentsRequest { id: id.into() })
            .await?
            .experiments
    };

    if let Some(experiment) = experiments.drain(..).next() {
        let experiment = convert!(experiment as _?);

        if show_jobs || show_reports {
            println!("{}", ui::Header::new("Experiment"));
        }

        print_experiment(&experiment);

        if show_jobs {
//            unimplemented!() @todo
        }

        if show_reports {
            if let Err(err) = print_reports(&mut system, &id).await {
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
    println!("{}", ui::ExperimentDetails::new(experiment));
}

async fn print_reports(system: &mut System, id: &str) -> Result<()> {
    println!("{}", ui::Header::new("Reports"));

    let reports = spinner! {
        system
            .client()
            .await?
            .find_reports(PFindReportsRequest { experiment_id: id.into() })
            .await?
            .reports
    };

    let reports = convert!(reports as [_?]);

    println!("{}", ui::ReportsTable::new(&reports));

    Ok(())
}