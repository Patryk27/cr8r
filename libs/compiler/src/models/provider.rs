use crate::Command;

pub type ProviderName = String;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDef {
    crate setup: Vec<Command>,
}

impl ProviderDef {
    pub fn new(setup: Vec<Command>) -> Self {
        Self { setup }
    }

    pub fn setup(&self) -> &[Command] {
        &self.setup
    }
}

#[cfg(test)]
impl Default for ProviderDef {
    fn default() -> Self {
        Self {
            setup: Vec::default(),
        }
    }
}