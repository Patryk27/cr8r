use lib_interop::protocol::core::PExperiment;
use lib_interop::protocol::for_client::{PFindExperimentReportsRequest, PFindExperimentsRequest};

use crate::{Result, spinner, System, ui};

pub async fn show(
    mut system: System,
    id: &str,
    show_steps: bool,
    show_reports: bool,
) -> Result<()> {
    let experiments = spinner! {
        system
            .client().await?
            .find_experiments(PFindExperimentsRequest { filter_id: id.into() }).await?
            .into_inner()
            .experiments
    };

    if let Some(experiment) = experiments.first() {
        if show_steps || show_reports {
            println!("{}", ui::Header::new("Experiment"));
        }

        print_experiment(experiment);

        if show_steps {
            print_steps(experiment);
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

fn print_experiment(experiment: &PExperiment) {
    println!("{}", ui::ExperimentDetails::new(experiment));
}

fn print_steps(experiment: &PExperiment) {
    println!("{}", ui::Header::new("Steps"));
    println!("{}", ui::ExperimentStepsTable::new(&experiment.steps));
}

async fn print_reports(system: &mut System, id: &str) -> Result<()> {
    println!("{}", ui::Header::new("Reports"));

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