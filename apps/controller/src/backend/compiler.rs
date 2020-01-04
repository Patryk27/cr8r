use anyhow::Context;

use lib_compiler::CompilerBuilder;
use lib_interop::domain::{DDefinition, DJob};

use crate::backend::Result;
use crate::config::{Ecosystem, Environment, Projects, Providers};

pub struct Compiler {
    compiler: lib_compiler::Compiler,
}

impl Compiler {
    pub fn new(ecosystem: Ecosystem) -> Result<Self> {
        let mut compiler = lib_compiler::Compiler::builder();

        setup_environment(&mut compiler, ecosystem.environment);

        setup_providers(&mut compiler, ecosystem.providers)
            .context("Could not setup providers")?;

        setup_projects(&mut compiler, ecosystem.projects)
            .context("Could not setup projects")?;

        Ok(Self {
            compiler: compiler.build()?,
        })
    }

    pub fn compile(&self, definition: &DDefinition) -> Vec<DJob> {
        self.compiler.compile(definition)
    }
}

fn setup_environment(compiler: &mut CompilerBuilder, environment: Environment) {
    compiler.environment(lib_compiler::Environment {
        default_toolchain: environment.default_toolchain,
    });
}

fn setup_providers(compiler: &mut CompilerBuilder, providers: Providers) -> Result<()> {
    for (provider_name, provider) in providers {
        let setup = provider.setup
            .into_iter()
            .map(lib_compiler::Command::new)
            .collect();

        let provider = lib_compiler::Provider::new(setup);

        compiler.add_provider(provider_name, provider)?;
    }

    Ok(())
}

fn setup_projects(compiler: &mut CompilerBuilder, projects: Projects) -> Result<()> {
    for (project_name, project) in projects {
        let project = lib_compiler::Project::new(
            project.repository,
            project.requirements,
        );

        compiler.add_project(project_name, project)?;
    }

    Ok(())
}