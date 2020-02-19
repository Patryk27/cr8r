use serde::Deserialize;

use crate::Command;

pub type StepName = String;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
    crate exec: Command,
}
