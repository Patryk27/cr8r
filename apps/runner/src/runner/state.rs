#[derive(Eq, PartialEq)]
pub enum RunnerState {
    Idle,

    Initializing,

    RunningExperiment {
        // @todo
    },
}