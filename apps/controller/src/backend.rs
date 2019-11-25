use crate::config;

pub use self::{
    compiler::*,
    runner::*,
    system::*,
};

mod compiler;
mod runner;
mod system;

pub fn start(ecosystem: config::Ecosystem, runner_secret: String) -> (actix::SystemRunner, System) {
    let actix = actix::System::new("cr8r");
    let compiler = Compiler::new(ecosystem);
    let system = System::spawn(runner_secret, compiler);

    (actix, system)
}