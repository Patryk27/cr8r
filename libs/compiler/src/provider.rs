use crate::Command;

pub type ProviderName = String;

#[derive(Debug)]
pub struct Provider {
    setup: Vec<Command>,
}

impl Provider {
    pub fn new(setup: Vec<Command>) -> Self {
        Self { setup }
    }

    pub fn setup(&self) -> &[Command] {
        &self.setup
    }
}