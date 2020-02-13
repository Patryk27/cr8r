#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExecutorStatus {
    Running,
    Completed,
    Stopped,
}