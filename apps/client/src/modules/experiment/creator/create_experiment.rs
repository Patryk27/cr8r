use anyhow::*;

use lib_core_channel::SendTo;
use lib_interop::domain::{DDefinition, DExperimentId};
use lib_interop::domain::definition::{DDependencyDef, DDependencySourceDef, DToolchainDef};

use crate::modules::definition::{DefinitionArg, DependencyArg, DependencySourceArg, ToolchainArg};

use super::{ExperimentCreator, ExperimentCreatorProgress::*};

impl<'c> ExperimentCreator<'c> {
    pub(super) async fn create_experiment(&mut self, definition: DefinitionArg) -> Result<DExperimentId> {
        CreatingExperiment
            .send_to(&self.progress);

        let experiment = self
            .build_def(definition)
            .into();

        let id: DExperimentId = self.ctxt
            .client()
            .await?
            .create_experiment(experiment)
            .await?
            .id
            .into();

        ExperimentCreated { id }
            .send_to(&self.progress);

        Ok(id)
    }

    fn build_def(&self, DefinitionArg { toolchain, dependencies }: DefinitionArg) -> DDefinition {
        let toolchain = self.build_toolchain_def(toolchain);

        let dependencies = dependencies
            .into_iter()
            .map(|dep| self.build_dependency_def(dep))
            .collect();

        DDefinition {
            toolchain,
            dependencies,
        }
    }

    fn build_toolchain_def(&self, toolchain: Option<ToolchainArg>) -> Option<DToolchainDef> {
        toolchain.map(|toolchain| {
            DToolchainDef {
                toolchain: toolchain.to_string(),
            }
        })
    }

    fn build_dependency_def(&self, dependency: DependencyArg) -> DDependencyDef {
        use DependencySourceArg::*;

        let name = dependency.name.0;

        let source = match dependency.source {
            Branch(branch) => {
                DDependencySourceDef::Branch { branch }
            }

            Tag(tag) => {
                DDependencySourceDef::Tag { tag }
            }

            Version(version) => {
                DDependencySourceDef::Version { version }
            }

            Path(_) => {
                DDependencySourceDef::Path {
                    attachment_id: self.attachments[&name].into(),
                }
            }
        };

        DDependencyDef { name, source }
    }
}