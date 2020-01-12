use crate::ProviderName;

pub type ProjectName = String;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectDef {
    crate repository: String,
    crate requirements: Vec<ProviderName>,
}

impl ProjectDef {
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

#[cfg(test)]
impl Default for ProjectDef {
    fn default() -> Self {
        Self {
            repository: "https://kernel.org".to_string(),
            requirements: Vec::default(),
        }
    }
}