use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{Environment, Project, ProjectName, Provider, ProviderName};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ecosystem {
    crate environment: Environment,

    #[serde(default)]
    crate providers: BTreeMap<ProviderName, Provider>,

    #[serde(default)]
    crate projects: BTreeMap<ProjectName, Project>,
}