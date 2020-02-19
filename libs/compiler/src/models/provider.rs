use serde::Deserialize;

use crate::Command;

pub type ProviderName = String;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provider {
    crate setup: Vec<Command>,
}

#[cfg(test)]
impl Default for Provider {
    fn default() -> Self {
        Self {
            setup: Default::default(),
        }
    }
}