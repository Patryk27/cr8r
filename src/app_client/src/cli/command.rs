use structopt::StructOpt;

use lib_protocol::ExperimentDefinition;

#[derive(Debug, StructOpt)]
#[structopt()]
pub enum Command {
    Controller(Controller),
    Experiment(Experiment),
}

#[derive(Debug, StructOpt)]
pub enum Controller {
    Status,
}

#[derive(Debug, StructOpt)]
pub enum Experiment {
    #[structopt()]
    Abort {
        experiment_id: String,
    },

    #[structopt()]
    Report {
        experiment_id: String,
    },

    Run(ExperimentDefinition),
}