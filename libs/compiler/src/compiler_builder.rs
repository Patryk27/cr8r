use std::collections::HashMap;

use crate::{Compiler, Environment, Project, ProjectName, Provider, ProviderName, Result};

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
        // @todo check for duplicates

        self.providers.insert(name, provider);

        Ok(())
    }

    pub fn add_project(&mut self, name: ProjectName, project: Project) -> Result<()> {
        // @todo check for duplicates

        self.projects.insert(name, project);

        Ok(())
    }

    pub fn build(self) -> Result<Compiler> {
        // @todo ensure projects <-> providers dependencies are met

        Ok(Compiler {
            defaults: self.environment.unwrap(),
            providers: self.providers,
            projects: self.projects,
        })
    }
}