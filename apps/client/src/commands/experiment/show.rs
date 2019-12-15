use lib_protocol::core::PExperiment;
use lib_protocol::for_client::{PFindExperimentReportsRequest, PFindExperimentsRequest};

use crate::{Result, spinner, System, ui};

pub async fn run(mut system: System, id: &str, show_reports: bool) -> Result<()> {
    let experiments = spinner! {
        system
            .client().await?
            .find_experiments(PFindExperimentsRequest { filter_id: id.into() }).await?
            .into_inner()
            .experiments
    };

    if let Some(experiment) = experiments.first() {
        print_experiment(experiment);

        if show_reports {
            if let Err(err) = print_reports(&mut system, &id).await {
                println!("Couldn't print reports");
            }
        }
    } else {
        println!("No such experiment exists");
    }

    Ok(())
}

fn print_experiment(experiment: &PExperiment) {
    // @todo print `# Experiment` header

    println!("{}", ui::Experiment::new(experiment));
}

async fn print_reports(system: &mut System, id: &str) -> Result<()> {
    let reports = spinner! {
        system
            .client().await?
            .find_experiment_reports(PFindExperimentReportsRequest { filter_experiment_id: id.into() }).await?
            .into_inner()
            .reports
    };

    println!("{}", ui::ExperimentReportsTable::new(&reports));

    Ok(())
}