#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Command {
    inner: String,
}

impl Command {
    pub fn new(inner: impl Into<String>) -> Self {
        Self { inner: inner.into() }
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }
}