use tonic::Status;

use anyhow::anyhow;
use lib_ui::Message;

pub struct Error;

impl Error {
    pub fn print(error: anyhow::Error) {
        let err_string = error.to_string();
        let mut err_overwritten = false;

        // `tonic` returns `tcp connect error: Connection refused` when we failed to connect to the controller; since
        // this is a pretty common case, we're handling it somewhat specially
        if err_string.contains("tcp connect error") {
            print_could_not_connect();
            err_overwritten = true;
        }

        // `tonic` returns `protocol error: stream no longer needed` when our request was terminated abnormally; since
        // the regular error message in this case is pretty vague, we're handling it somewhat specially
        if err_string.contains("stream no longer needed") {
            print_controller_crashed();
            err_overwritten = true;
        }

        if err_overwritten {
            eprint!("{}", Message::info(
                "For reference, the underlying error was:",
                format!("{:?}", error),
            ));

            return;
        }

        match error.downcast::<Status>() {
            Ok(err) => {
                eprint!("{}", Message::error("Error:", err.message()));
            }

            Err(err) => {
                eprint!("{}", lib_ui::Error::new(&err));
            }
        }
    }
}

fn print_could_not_connect() {
    eprintln!("{}", lib_ui::Error::new(&anyhow!(
        "Could not connect to controller"
    )));

    eprintln!("{}", Message::warn(
        "Note:",
        "This is most likely caused by a misconfiguration in your `client.toml`",
    ));
}

fn print_controller_crashed() {
    eprintln!("{}", lib_ui::Error::new(&anyhow!(
        "Controller returned no response"
    )));

    eprintln!("{}", Message::warn(
        "Note:",
        [
            "This is most likely caused by a bug in the controller.",
            "",
            "Please try repeating the latest action and if the problem persists, controller's",
            "log should tell you some useful information."
        ].join("\n"),
    ));
}
