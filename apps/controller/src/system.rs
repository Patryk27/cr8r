use std::sync::Arc;

use bastion::Bastion;

use crate::Ecosystem;

pub use self::{
    compiler::*,
    error::*,
    experiment_session::*,
    runner_session::*,
    system::*,
};

mod compiler;
mod error;
mod experiment_session;
mod runner_session;
mod system;

pub fn start(runner_secret: RunnerSecret, ecosystem: Ecosystem) -> System {
    Bastion::init();

    let runner_secret = Arc::new(runner_secret);

    // Initialize compiler
    let compiler = Arc::new(
        Compiler::new(ecosystem),
    );

    // Initialize application
    let children = Bastion::children(|children| {
        children.with_exec(move |ctx| {
            System::start(
                runner_secret.clone(),
                compiler.clone(),
                ctx,
            )
        })
    }).unwrap();

    // Takeoff!
    Bastion::start();

    children
        .elems()[0]
        .clone()
        .into()
}