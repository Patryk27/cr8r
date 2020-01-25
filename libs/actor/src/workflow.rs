#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActorWorkflow {
    Continue,
    Stop,
}

impl ActorWorkflow {
    /// Returns whether this workflow is `ActorWorkflow::Continue`.
    pub fn should_continue(self) -> bool {
        match self {
            ActorWorkflow::Continue => true,
            _ => false,
        }
    }

    /// Returns whether this workflow is `ActorWorkflow::Stop`.
    pub fn should_stop(self) -> bool {
        match self {
            ActorWorkflow::Stop => true,
            _ => false,
        }
    }
}