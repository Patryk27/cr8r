use std::collections::HashMap;

use lib_interop::contract::{CExperimentDefinition, CJob};

use crate::{CompilerBuilder, Defaults, Project, ProjectName, Provider, ProviderName};

mod compilers;

pub struct Compiler {
    pub(crate) defaults: Defaults,
    pub(crate) providers: HashMap<ProviderName, Provider>,
    pub(crate) projects: HashMap<ProjectName, Project>,
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

            OverrideCrate { .. } => {
                unimplemented!()
            }

            PatchCrate { .. } => {
                unimplemented!()
            }
        }
    }
}