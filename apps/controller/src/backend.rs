use lib_protocol::core::RunnerSecret;

use crate::core::Ecosystem;

pub use self::{
    compiler::*,
    error::*,
    experiment::*,
    runner::*,
    system::*,
};

mod compiler;
mod error;
mod experiment;
mod runner;
mod system;

pub fn start(runner_secret: RunnerSecret, ecosystem: Ecosystem) -> System {
    let compiler = Compiler::new(ecosystem);

    System::spawn(runner_secret, compiler)
}

// IntelliJ's Rust plugin doesn't yet understand how to properly format declarative macros, so we're disabling it here
// @formatter:off
macro msg {
    ($self_tx:expr, $msg:expr) => {{
        if !$self_tx.unbounded_send($msg).is_ok() {
            panic!("Failed to send message to the actor - did it die prematurely?"); // @todo
        }
    }},

    ($self_tx:expr, $tx:ident, $msg:expr) => {{
        let ($tx, rx) = futures_channel::oneshot::channel();

        if $self_tx.unbounded_send($msg).is_ok() {
            if let Ok(rx) = rx.await {
                rx
            } else {
                panic!("Failed to await actor's response - did it die prematurely?"); // @todo
            }
        } else {
            panic!("Failed to send message to the actor - did it die prematurely?"); // @todo
        }
    }},
}

macro uuid () {
    uuid::Uuid::new_v4()
        .to_hyphenated()
        .to_string()
}
// @formatter:on