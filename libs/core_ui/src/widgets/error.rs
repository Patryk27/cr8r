use std::fmt;

use crate::MessageWidget;

pub struct ErrorWidget<'a> {
    error: &'a anyhow::Error,
}

impl<'a> ErrorWidget<'a> {
    pub fn new(error: &'a anyhow::Error) -> Self {
        Self { error }
    }
}

impl fmt::Display for ErrorWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", MessageWidget::error_inv("Error:", self.error.to_string()))?;

        let causes = self
            .error
            .chain()
            .skip(1);

        for cause in causes {
            writeln!(f)?;
            write!(f, "{}", MessageWidget::warn_inv("Caused by:", cause.to_string()))?;
        }

        Ok(())
    }
}
