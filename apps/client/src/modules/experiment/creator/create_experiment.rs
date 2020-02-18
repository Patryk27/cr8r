use anyhow::*;

use lib_core_channel::SendTo;
use lib_interop::models::{DDefinition, DExperimentId};
use lib_interop::models::definition::{DDependencyDef, DDependencySourceDef, DToolchainDef};

use crate::modules::definition::{DefinitionArg, DependencyArg, DependencySourceArg, ToolchainArg};

use super::{ExperimentCreator, ExperimentCreatorProgress::*};

impl ExperimentCreator {
    pub(super) async fn create_experiment(&mut self, definition: DefinitionArg) -> Result<DExperimentId> {
        CreatingExperiment
            .send_to(&self.progress);

        let definition = self.build_def(definition);

        let id = self.conn
            .experiments()
            .create(definition)
            .await?;

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

    fn build_dependency_def(&self, dep: DependencyArg) -> DDependencyDef {
        use DependencySourceArg::*;

        let name = dep.name.0;

        let source = match dep.source {
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