use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{Command, ProviderName, Step, StepName};

pub type ProjectName = String;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    crate source: String,

    #[serde(default)]
    crate requirements: Vec<ProviderName>,

    #[serde(default = "default_steps")]
    crate steps: BTreeMap<StepName, Step>,
}

#[cfg(test)]
impl Default for Project {
    fn default() -> Self {
        Self {
            source: "https://kernel.org".to_string(),
            requirements: Default::default(),
            steps: Default::default(),
        }
    }
}

fn default_steps() -> BTreeMap<StepName, Step> {
    let check = ("cargo check".to_string(), Step {
        exec: Command {
            inner: "cargo check --all --all-targets".to_string(),
        },
    });

    let test = ("cargo test".to_string(), Step {
        exec: Command {
            inner: "cargo test --all --all-targets".to_string(),
        },
    });

    vec![
        check,
        test,
    ].into_iter().collect()
}
