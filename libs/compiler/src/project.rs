use crate::ProviderName;

pub type ProjectName = String;

pub struct Project {
    repository: String,
    requirements: Vec<ProviderName>,
}

impl Project {
    pub fn new(repository: String, requirements: Vec<ProviderName>) -> Self {
        Self { repository, requirements }
    }
}