use std::collections::HashMap;

use lib_interop::contract::{CExperimentDefinition, CJob};

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

    pub fn compile(&self, experiment_def: &CExperimentDefinition) -> Vec<CJob> {
        use CExperimentDefinition::*;

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