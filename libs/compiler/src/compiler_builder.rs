use std::collections::hash_map::Entry;
use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::{Compiler, Environment, Project, ProjectName, Provider, ProviderName};

#[derive(Default)]
pub struct CompilerBuilder {
    environment: Option<Environment>,
    providers: HashMap<ProviderName, Provider>,
    projects: HashMap<ProjectName, Project>,
}

impl CompilerBuilder {
    pub fn environment(&mut self, defaults: Environment) {
        self.environment = Some(defaults);
    }

    pub fn add_provider(&mut self, name: ProviderName, provider: Provider) -> Result<()> {
        match self.providers.entry(name) {
            Entry::Occupied(entry) => {
                Err(anyhow!("Provider `{}` has been already added into the compiler", entry.key()))
            }

            Entry::Vacant(entry) => {
                entry.insert(provider);
                Ok(())
            }
        }
    }

    pub fn add_project(&mut self, name: ProjectName, project: Project) -> Result<()> {
        match self.projects.entry(name) {
            Entry::Occupied(entry) => {
                Err(anyhow!("Project `{}` has been already added into the compiler", entry.key()))
            }

            Entry::Vacant(entry) => {
                entry.insert(project);
                Ok(())
            }
        }
    }

    pub fn build(self) -> Result<Compiler> {
        let environment = self.environment.ok_or_else(|| {
            anyhow!("Environment has not been configured")
        })?;

        Ok(Compiler {
            environment,
            providers: self.providers,
            projects: self.projects,
        })
    }
}