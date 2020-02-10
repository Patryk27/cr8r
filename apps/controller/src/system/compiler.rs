use anyhow::*;

use lib_compiler::CompilerBuilder;
use lib_interop::domain::{DDefinition, DJob};

use crate::system::{EcosystemConfig, EcosystemEnvironmentConfig, EcosystemProjectsConfig, EcosystemProvidersConfig};

pub struct Compiler {
    compiler: lib_compiler::Compiler,
}

impl Compiler {
    pub fn new(ecosystem: EcosystemConfig) -> Result<Self> {
        let mut compiler = lib_compiler::Compiler::builder();

        setup_environment(&mut compiler, ecosystem.environment);
        setup_providers(&mut compiler, ecosystem.providers)?;
        setup_projects(&mut compiler, ecosystem.projects)?;

        Ok(Self {
            compiler: compiler.build()?,
        })
    }

    pub fn compile(&self, definition: &DDefinition) -> Vec<DJob> {
        self.compiler.compile(definition)
    }
}

fn setup_environment(compiler: &mut CompilerBuilder, environment: EcosystemEnvironmentConfig) {
    let environment = lib_compiler::Environment::new(environment.default_toolchain);

    compiler.set_environment(environment);
}

fn setup_providers(compiler: &mut CompilerBuilder, providers: EcosystemProvidersConfig) -> Result<()> {
    for (provider_name, provider) in providers {
        let setup = provider.setup
            .into_iter()
            .map(lib_compiler::Command::new)
            .collect();

        let provider = lib_compiler::ProviderDef::new(setup);

        compiler.add_provider(provider_name, provider)?;
    }

    Ok(())
}

fn setup_projects(compiler: &mut CompilerBuilder, projects: EcosystemProjectsConfig) -> Result<()> {
    for (project_name, project) in projects {
        let project = lib_compiler::ProjectDef::new(
            project.repository,
            project.requirements,
        );

        compiler.add_project(project_name, project)?;
    }

    Ok(())
}