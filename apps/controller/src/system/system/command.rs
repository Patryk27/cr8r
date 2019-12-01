use lib_protocol::core::experiment_definition::ExperimentDefinitionInner;

use crate::system::{ExperimentId, RunnerId, RunnerName, RunnerSecret};

#[derive(Debug)]
pub enum Command {
    AbortExperiment {
        experiment: ExperimentId,
    },

    LaunchExperiment {
        experiment: ExperimentDefinitionInner,
    },

    ReportExperiment {
        runner: RunnerId,
        experiment: ExperimentId,
        report: (), // @todo
    },

    RequestExperiment {
        runner: RunnerId,
    },

    RegisterRunner {
        name: RunnerName,
        secret: RunnerSecret,
    },
}