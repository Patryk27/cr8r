use thiserror::*;

use crate::{ProjectName, ProviderName};

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CompilerBuilderError {
    #[error("Project `{project_name}` depends on unknown provider `{provider_name}`")]
    ProjectContainsMissingDependency {
        project_name: ProjectName,
        provider_name: ProviderName,
    },
}
