use anyhow::*;
use tokio::fs::metadata;

use lib_core_channel::SendTo;

use crate::modules::definition::{DefinitionArg, DependencyArg, DependencySourceArg};

use super::{ExperimentCreator, ExperimentCreatorProgress::*};

impl ExperimentCreator {
    pub(super) async fn validate_dependencies(&mut self, definition: &DefinitionArg) -> Result<()> {
        if definition.dependencies.is_empty() {
            return Ok(());
        }

        ValidatingDependencies.send_to(&self.progress);

        for dep in &definition.dependencies {
            ValidatingDependency {
                name: dep.name.to_string(),
            }.send_to(&self.progress);

            self.validate_dependency(dep).await?;
        }

        DependenciesValidated
            .send_to(&self.progress);

        Ok(())
    }

    async fn validate_dependency(&mut self, dep: &DependencyArg) -> Result<()> {
        use DependencySourceArg::*;

        if let Path(path) = &dep.source {
            if metadata(path).await.is_err() {
                return Err(anyhow!(
                    "Dependency `{}` refers to a non-existing path `{}`",
                    dep.name, path,
                ));
            }
        }

        Ok(())
    }
}