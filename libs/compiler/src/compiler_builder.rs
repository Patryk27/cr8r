use std::collections::HashMap;

use crate::{Compiler, Defaults, Project, ProjectName, Provider, ProviderName, Result};

#[derive(Default)]
pub struct CompilerBuilder {
    defaults: Option<Defaults>,
    providers: HashMap<ProviderName, Provider>,
    projects: HashMap<ProjectName, Project>,
}

impl CompilerBuilder {
    pub fn defaults(&mut self, defaults: Defaults) {
        self.defaults = Some(defaults);
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
        // @todo ensure projects <-> providers dependencies make sense

        Ok(Compiler {
            defaults: self.defaults.unwrap(),
            providers: self.providers,
            projects: self.projects,
        })
    }
}