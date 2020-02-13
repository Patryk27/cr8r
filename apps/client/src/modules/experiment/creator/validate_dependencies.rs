use anyhow::*;
use tokio::fs::metadata;

use lib_core_channel::SendTo;

use crate::modules::definition::{DefinitionArg, DependencyArg, DependencySourceArg};

use super::{ExperimentCreator, ExperimentCreatorProgress::*};

impl<'c> ExperimentCreator<'c> {
    pub(super) async fn validate_dependencies(&mut self, definition: &DefinitionArg) -> Result<()> {
        if definition.dependencies.is_empty() {
            return Ok(());
        }

        ValidatingDependencies.send_to(&self.progress);

        for dependency in &definition.dependencies {
            ValidatingDependency {
                name: dependency.name.to_string(),
            }.send_to(&self.progress);

            self.validate_dependency(dependency)
                .await?;
        }

        DependenciesValidated
            .send_to(&self.progress);

        Ok(())
    }

    async fn validate_dependency(&mut self, dependency: &DependencyArg) -> Result<()> {
        use DependencySourceArg::*;

        match &dependency.source {
            Path(path) => {
                if metadata(path).await.is_err() {
                    return Err(anyhow!(
                        "Dependency `{}` refers to a non-existing path `{}`",
                        dependency.name, path,
                    ));
                }
            }

            _ => (),
        }

        Ok(())
    }
}