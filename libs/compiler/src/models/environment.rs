#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Environment {
    crate default_toolchain: String,
}

impl Environment {
    pub fn new(default_toolchain: String) -> Self {
        Self { default_toolchain }
    }

    pub fn default_toolchain(&self) -> &str {
        &self.default_toolchain
    }
}

#[cfg(test)]
impl Default for Environment {
    fn default() -> Self {
        Self {
            default_toolchain: "nightly".into(),
        }
    }
}