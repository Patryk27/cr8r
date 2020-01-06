use std::fmt;

use crate::Message;

pub struct Error<'a> {
    error: &'a anyhow::Error,
}

impl<'a> Error<'a> {
    pub fn new(error: &'a anyhow::Error) -> Self {
        Self { error }
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Message::error("Error:", self.error.to_string()))?;

        let causes = self
            .error
            .chain()
            .skip(1);

        for cause in causes {
            writeln!(f)?;
            write!(f, "{}", Message::warn("Caused by:", cause.to_string()))?;
        }

        Ok(())
    }
}
