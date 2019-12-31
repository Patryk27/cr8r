use crate::ProviderName;

pub type ProjectName = String;

#[derive(Debug)]
pub struct Project {
    repository: String,
    requirements: Vec<ProviderName>,
}

impl Project {
    pub fn new(repository: String, requirements: Vec<ProviderName>) -> Self {
        Self { repository, requirements }
    }

    pub fn repository(&self) -> &str {
        &self.repository
    }

    pub fn requirements(&self) -> &[ProviderName] {
        &self.requirements
    }
}