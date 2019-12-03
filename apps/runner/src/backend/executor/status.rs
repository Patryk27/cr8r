#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExecutorStatus {
    Aborted,
    Completed,
    Running,
}