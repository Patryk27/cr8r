use thiserror::*;

use crate::{ProjectName, ProviderName};

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CompilerBuilderError {
    #[error("Environment has not been defined")]
    EnvironmentNotDefined,

    #[error("Provider `{name}` has been already defined")]
    ProviderAlreadyDefined {
        name: ProviderName,
    },

    #[error("Project `{name}` has been already defined")]
    ProjectAlreadyDefined {
        name: ProjectName,
    },

    #[error("Project `{project_name}` depends on provider `{provider_name}`, which has not been defined")]
    ProjectContainsMissingDependency {
        project_name: ProjectName,
        provider_name: ProviderName,
    },
}
