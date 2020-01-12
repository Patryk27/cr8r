#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExperimentExecutorStatus {
    Aborted,
    Completed,
    Running,
}