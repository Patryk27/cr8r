use lib_interop::contract::{CExperimentDef, CProgram};

use crate::backend::Result;
use crate::core::Ecosystem;

pub struct Compiler {
    compiler: lib_compiler::Compiler,
}

impl Compiler {
    pub fn new(ecosystem: Ecosystem) -> Result<Self> {
        let mut compiler = lib_compiler::Compiler::builder();

        compiler.defaults(lib_compiler::Defaults {
            system: ecosystem.environment.default_system,
            toolchain: ecosystem.environment.default_toolchain,
        });

        for (provider_name, provider) in ecosystem.flora {
            let setup = provider.setup
                .into_iter()
                .map(lib_compiler::Command::new)
                .collect();

            let provider = lib_compiler::Provider::new(setup);

            compiler.add_provider(provider_name, provider)?;
        }

        for (project_name, project) in ecosystem.fauna {
            let project = lib_compiler::Project::new(
                project.repository,
                project.requirements,
            );

            compiler.add_project(project_name, project)?;
        }

        Ok(Self {
            compiler: compiler.build()?,
        })
    }

    pub fn compile(&self, def: &CExperimentDef) -> CProgram {
        self.compiler.compile(def)
    }
}