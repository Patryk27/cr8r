pub struct Command {
    inner: String,
}

impl Command {
    pub fn new(inner: String) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }
}