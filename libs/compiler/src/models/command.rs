use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct Command {
    crate inner: String,
}

impl Command {
    pub fn inner(&self) -> &str {
        &self.inner
    }
}