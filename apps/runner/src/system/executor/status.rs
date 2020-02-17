#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExecutorStatus {
    Running,
    Completed,
    Stopped,
}

impl Default for ExecutorStatus {
    fn default() -> Self {
        ExecutorStatus::Running
    }
}