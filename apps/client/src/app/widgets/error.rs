use anyhow::*;
use tonic::Status;

use lib_core_ui::*;

pub struct AppErrorWidget;

impl AppErrorWidget {
    pub fn print(error: anyhow::Error) {
        let err_string = error.to_string();

        // There exist a few errors that are more likely to occur in reality than the other ones (e.g. `Connection
        // refused`) - for such errors we have prepared custom `print_*` functions that provide more context than the
        // generic error message
        let mut err_overwritten = false;

        if err_string.contains("Connection refused") {
            print_connection_failed();
            err_overwritten = true;
        }

        if err_string.contains("broken pipe") {
            print_connection_lost();
            err_overwritten = true;
        }

        if err_string.contains("stream no longer needed") {
            print_connection_dropped();
            err_overwritten = true;
        }

        if err_overwritten {
            eprint!("{}", MessageWidget::info(
                "For reference, the underlying error was:",
                format!("{:?}", error),
            ));

            return;
        }

        match error.downcast::<Status>() {
            Ok(err) => {
                eprint!("{}", MessageWidget::error("Error:", err.message()));
            }

            Err(err) => {
                eprint!("{}", ErrorWidget::new(&err));
            }
        }
    }
}

fn print_connection_failed() {
    eprintln!("{}", ErrorWidget::new(&anyhow!(
        "Could not connect to the controller"
    )));

    eprintln!("{}", MessageWidget::warn(
        "Note:",
        [
            "This is most likely caused by a misconfiguration in the `client.yaml` file.",
            "",
            "Please ensure that all URLs and credentials are valid, that the controller is",
            "actually running and it's accessible from your network.",
        ].join("\n"),
    ));
}

fn print_connection_lost() {
    eprintln!("{}", ErrorWidget::new(&anyhow!(
        "Lost connection to the controller"
    )));

    eprintln!("{}", MessageWidget::warn(
        "Note:",
        [
            "This might happen because of a network partitioning (e.g. you or the controller",
            "lost access to the network) or because the controller has been manually shut",
            "down.",
            "",
            "Please try repeating the latest action and, if the problem persists, you should",
            "find some useful information in the controller's log."
        ].join("\n"),
    ));
}

fn print_connection_dropped() {
    eprintln!("{}", ErrorWidget::new(&anyhow!(
        "Could not read controller's response"
    )));

    eprintln!("{}", MessageWidget::warn(
        "Note:",
        [
            "This is most likely caused by a bug in the controller.",
            "",
            "Please try repeating the latest action and, if the problem persists, you should",
            "find some useful information in the controller's log."
        ].join("\n"),
    ));
}
