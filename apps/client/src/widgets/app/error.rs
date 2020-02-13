use anyhow::*;

use lib_core_ui::*;

pub struct AppErrorWidget;

impl AppErrorWidget {
    pub fn print(error: anyhow::Error) {
        let error_msg = error.to_string();

        // There exist a few errors that are more likely to occur in reality than the other ones (e.g. `Connection
        // refused`) - for such errors we have prepared custom `print_*` functions that provide more context than the
        // generic error message
        let mut error_handled = false;

        if error_msg.contains("Connection refused") {
            print_connection_failed();
            error_handled = true;
        }

        if error_msg.contains("broken pipe") {
            print_connection_lost();
            error_handled = true;
        }

        if error_msg.contains("stream no longer needed") {
            print_connection_dropped();
            error_handled = true;
        }

        if error_handled {
            MessageWidget::info_inv(
                "For reference, the underlying error was:",
                format!("{:?}", error),
            ).eprint();
        } else {
            ErrorWidget::new(&error)
                .eprint();
        }
    }
}

fn print_connection_failed() {
    ErrorWidget::new(&anyhow!("Could not connect to the controller"))
        .eprintln();

    MessageWidget::warn_inv("Note:", [
        "This is most likely caused by a misconfiguration in the `client.yaml` file.",
        "",
        "Please ensure that all URLs and credentials are valid, that the controller is",
        "actually running and it's accessible from your network.",
    ]).eprintln();
}

fn print_connection_lost() {
    ErrorWidget::new(&anyhow!("Lost connection to the controller"))
        .eprintln();

    MessageWidget::warn_inv("Note:", [
        "This might happen because of a network partitioning (e.g. you or the controller",
        "lost access to the network) or because the controller has been manually shut",
        "down.",
        "",
        "Please try repeating the latest action and, if the problem persists, you should",
        "find some useful information in the controller's log."
    ]).eprintln();
}

fn print_connection_dropped() {
    ErrorWidget::new(&anyhow!("Could not read controller's response"))
        .eprintln();

    MessageWidget::warn_inv("Note:", [
        "This is most likely caused by a bug in the controller.",
        "",
        "Please try repeating the latest action and, if the problem persists, you should",
        "find some useful information in the controller's log."
    ]).eprintln();
}
