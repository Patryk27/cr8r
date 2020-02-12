#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub enum ActorWorkflow {
    Continue,
    Stop,
}

impl ActorWorkflow {
    /// Returns whether this workflow is `ActorWorkflow::Continue`.
    pub fn actor_should_continue(self) -> bool {
        match self {
            ActorWorkflow::Continue => true,
            _ => false,
        }
    }

    /// Returns whether this workflow is `ActorWorkflow::Stop`.
    pub fn actor_should_stop(self) -> bool {
        match self {
            ActorWorkflow::Stop => true,
            _ => false,
        }
    }
}