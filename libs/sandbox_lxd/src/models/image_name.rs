use std::fmt;

#[derive(Clone, Debug)]
pub struct LxdImageName(String);

impl LxdImageName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for LxdImageName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Display for LxdImageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}