use std::collections::HashMap;

use lib_interop::domain::{DDefinition, DJob};

use crate::{CompilerBuilder, Environment, Project, ProjectName, Provider, ProviderName};

mod compilers;

#[derive(Debug)]
pub struct Compiler {
    crate environment: Environment,
    crate providers: HashMap<ProviderName, Provider>,
    crate projects: HashMap<ProjectName, Project>,
}

impl Compiler {
    pub fn builder() -> CompilerBuilder {
        CompilerBuilder::default()
    }

    pub fn compile(&self, definition: &DDefinition) -> Vec<DJob> {
        unimplemented!()
    }
}