use std::collections::HashMap;

use lib_interop::domain::{DExperimentDefinition, DJob};

use crate::{CompilerBuilder, Environment, Project, ProjectName, Provider, ProviderName};

mod compilers;

#[derive(Debug)]
pub struct Compiler {
    crate defaults: Environment,
    crate providers: HashMap<ProviderName, Provider>,
    crate projects: HashMap<ProjectName, Project>,
}

impl Compiler {
    pub fn builder() -> CompilerBuilder {
        CompilerBuilder::default()
    }

    pub fn compile(&self, experiment_def: &DExperimentDefinition) -> Vec<DJob> {
        use DExperimentDefinition::*;

        match experiment_def {
            OverrideToolchain { toolchain } => {
                compilers::compile_override_toolchain(self, toolchain)
            }

            OverrideCrate { name, version } => {
                compilers::compile_override_crate(self, name, version)
            }

            PatchCrate { .. } => {
                unimplemented!()
            }
        }
    }
}