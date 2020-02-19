use crate::{Compiler, Ecosystem};

pub use self::error::CompilerBuilderError;

mod error;

#[derive(Default)]
pub struct CompilerBuilder {
    ecosystem: Option<Ecosystem>,
}

impl CompilerBuilder {
    pub fn set_ecosystem(&mut self, ecosystem: Ecosystem) {
        self.ecosystem = Some(ecosystem);
    }

    pub fn build(self) -> Result<Compiler, CompilerBuilderError> {
        let ecosystem = self.ecosystem.unwrap();

        for (project_name, project) in &ecosystem.projects {
            for project_requirement in &project.requirements {
                if !ecosystem.providers.contains_key(project_requirement) {
                    return Err(CompilerBuilderError::ProjectContainsMissingDependency {
                        project_name: project_name.to_owned(),
                        provider_name: project_requirement.to_owned(),
                    });
                }
            }
        }

        Ok(Compiler {
            environment: ecosystem.environment,
            providers: ecosystem.providers,
            projects: ecosystem.projects,
        })
    }
}
