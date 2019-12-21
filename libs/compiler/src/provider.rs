use crate::Command;

pub type ProviderName = String;

pub struct Provider {
    setup: Vec<Command>,
}

impl Provider {
    pub fn new(setup: Vec<Command>) -> Self {
        Self { setup }
    }

    pub fn setup(&self) -> &Vec<Command> {
        &self.setup
    }
}