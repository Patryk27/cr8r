use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

use crate::system::{ExperimentId, RunnerName, RunnerSecret, RunnerToken};

#[derive(Debug)]
pub enum Command {
    AbortExperiment {
        experiment: ExperimentId,
    },

    LaunchExperiment {
        experiment: ExperimentDefinitionInner,
    },

    ReportExperiment {
        runner: RunnerToken,
        experiment: ExperimentId,
        report: (), // @todo
    },

    RequestExperiment {
        runner: RunnerToken,
    },

    RegisterRunner {
        name: RunnerName,
        secret: RunnerSecret,
    },
}