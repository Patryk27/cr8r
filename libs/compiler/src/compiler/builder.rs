use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::{Compiler, Environment, ProjectDef, ProjectName, ProviderDef, ProviderName};

pub use self::error::CompilerBuilderError;

mod error;

#[derive(Default)]
pub struct CompilerBuilder {
    environment: Option<Environment>,
    providers: HashMap<ProviderName, ProviderDef>,
    projects: HashMap<ProjectName, ProjectDef>,
}

impl CompilerBuilder {
    pub fn set_environment(&mut self, environment: Environment) {
        self.environment = Some(environment);
    }

    pub fn add_provider(
        &mut self,
        name: impl Into<ProviderName>,
        def: ProviderDef,
    ) -> Result<(), CompilerBuilderError> {
        let name = name.into();

        match self.providers.entry(name) {
            Entry::Occupied(entry) => {
                Err(CompilerBuilderError::ProviderAlreadyDefined {
                    name: entry.key().to_owned(),
                })
            }

            Entry::Vacant(entry) => {
                entry.insert(def);
                Ok(())
            }
        }
    }

    pub fn add_project(
        &mut self,
        name: impl Into<ProjectName>,
        def: ProjectDef,
    ) -> Result<(), CompilerBuilderError> {
        let name = name.into();

        match self.projects.entry(name) {
            Entry::Occupied(entry) => {
                Err(CompilerBuilderError::ProjectAlreadyDefined {
                    name: entry.key().to_owned(),
                })
            }

            Entry::Vacant(entry) => {
                entry.insert(def);
                Ok(())
            }
        }
    }

    pub fn build(self) -> Result<Compiler, CompilerBuilderError> {
        let environment = self.environment.ok_or_else(|| CompilerBuilderError::EnvironmentNotDefined)?;

        for (project_name, project) in &self.projects {
            for project_requirement in &project.requirements {
                if !self.providers.contains_key(project_requirement) {
                    return Err(CompilerBuilderError::ProjectContainsMissingDependency {
                        project_name: project_name.to_owned(),
                        provider_name: project_requirement.to_owned(),
                    });
                }
            }
        }

        Ok(Compiler {
            environment,
            providers: self.providers,
            projects: self.projects,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_providers_cannot_be_duplicated() {
        let mut builder = CompilerBuilder::default();

        assert_eq!(
            builder.add_provider("foo", ProviderDef::default()),
            Ok(())
        );

        assert_eq!(
            builder.add_provider("bar", ProviderDef::default()),
            Ok(())
        );

        assert_eq!(
            builder.add_provider("foo", ProviderDef::default()),
            Err(CompilerBuilderError::ProviderAlreadyDefined { name: "foo".to_string() })
        );
    }

    #[test]
    fn ensure_projects_cannot_be_duplicated() {
        let mut builder = CompilerBuilder::default();

        assert_eq!(
            builder.add_project("foo", ProjectDef::default()),
            Ok(())
        );

        assert_eq!(
            builder.add_project("bar", ProjectDef::default()),
            Ok(())
        );

        assert_eq!(
            builder.add_project("foo", ProjectDef::default()),
            Err(CompilerBuilderError::ProjectAlreadyDefined { name: "foo".to_string() })
        );
    }

    #[test]
    fn ensure_environment_must_be_defined() {
        let builder = CompilerBuilder::default();

        assert_eq!(
            builder.build(),
            Err(CompilerBuilderError::EnvironmentNotDefined)
        );
    }

    #[test]
    fn ensure_projects_must_refer_to_existing_providers() {
        let mut builder = CompilerBuilder::default();

        builder.set_environment(Environment::default());

        builder
            .add_provider("x", ProviderDef::default())
            .unwrap();

        builder
            .add_provider("y", ProviderDef::default())
            .unwrap();

        builder
            .add_project("foo", ProjectDef {
                repository: "git.foo.org".into(),
                requirements: vec!["x".into(), "y".into()],
            })
            .unwrap();

        builder
            .add_project("bar", ProjectDef {
                repository: "git.bar.org".into(),
                requirements: vec!["x".into(), "z".into()],
            })
            .unwrap();

        assert_eq!(
            builder.build(),
            Err(CompilerBuilderError::ProjectContainsMissingDependency {
                project_name: "bar".into(),
                provider_name: "z".into(),
            })
        );
    }
}