use std::collections::HashMap;

use lib_interop::contract::{CExperimentDef, CProgram};

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

    pub fn compile(&self, experiment_def: &CExperimentDef) -> CProgram {
        use CExperimentDef::*;

        match experiment_def {
            TryPatchCrate { name, attachment_id } => {
                compilers::compile_try_patch_crate(self, name, attachment_id.as_str())
            }

            TryToolchain { toolchain } => {
                compilers::compile_try_toolchain(self, toolchain)
            }
        }
    }
}