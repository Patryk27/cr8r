use core::result;
use std::fmt;

use anyhow::*;
use itertools::Itertools;

use crate::{MessageWidget, Widget};

pub struct ErrorWidget {
    error: Error,
}

impl ErrorWidget {
    pub fn new(error: Error) -> Self {
        Self { error }
    }
}

impl fmt::Display for ErrorWidget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if handle_common_error(f, &self.error)? {
            MessageWidget::info_inv(
                "For reference, the underlying error was:",
                format!("{:?}", self.error),
            ).write(f)?;
        } else {
            MessageWidget::error_inv("Error:", self.error.to_string())
                .write(f)?;

            for cause in self.error.chain().skip(1) {
                writeln!(f)?;

                MessageWidget::warn_inv("Caused by:", cause.to_string())
                    .write(f)?;
            }
        }

        Ok(())
    }
}

/// There exist some errors (like "Connection refused") that are more likely to occur than to the other ones - for such,
/// we've prepared hand-made error messages providing more insight into what's happening
fn handle_common_error(f: &mut fmt::Formatter<'_>, error: &Error) -> result::Result<bool, fmt::Error> {
    // This heuristic is absurdly basic, but gets the job done 99% times

    let error = error
        .chain()
        .map(ToString::to_string)
        .join("\n");

    if error.contains("Connection refused") {
        print_connection_failed(f)?;
        return Ok(true);
    }

    if error.contains("broken pipe") {
        print_connection_lost(f)?;
        return Ok(true);
    }

    if error.contains("stream no longer needed") {
        print_connection_dropped(f)?;
        return Ok(true);
    }

    Ok(false)
}

fn print_connection_failed(f: &mut fmt::Formatter<'_>) -> fmt::Result {
    MessageWidget::error_inv(
        "Error:",
        "Could not connect to the controller",
    ).writeln(f)?;

    MessageWidget::warn_inv("Note:", [
        "This may be caused by a misconfiguration the `.yaml` file.",
        "",
        "Please ensure controller's URL and credentials are valid and that controller is",
        "accessible from your network.",
    ]).writeln(f)
}

fn print_connection_lost(f: &mut fmt::Formatter<'_>) -> fmt::Result {
    MessageWidget::error_inv(
        "Error:",
        "Lost connection to the controller",
    ).writeln(f)?;

    MessageWidget::warn_inv("Note:", [
        "This may be caused by the controller being suddenly shut down.",
        "",
        "Please try repeating the latest action and, if the problem persists, you should",
        "find some useful information in the controller's log.",
    ]).writeln(f)
}

fn print_connection_dropped(f: &mut fmt::Formatter<'_>) -> fmt::Result {
    MessageWidget::error_inv(
        "Error:",
        "Could not read controller's response",
    ).writeln(f)?;

    MessageWidget::warn_inv("Note:", [
        "This may be caused by a bug in the controller.",
        "",
        "Please try repeating the latest action and, if the problem persists, you should",
        "find some useful information in the controller's log."
    ]).writeln(f)
}
