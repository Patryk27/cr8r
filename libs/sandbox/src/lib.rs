#![feature(box_syntax)]
#![feature(crate_visibility_modifier)]

pub use self::{
    def::*,
    engine::*,
    engines::*,
    error::*,
    listener::*,
    mount::*,
    provider::*,
};

mod def;
mod engine;
mod engines;
mod error;
mod listener;
mod mount;
mod provider;

pub struct Sandbox {
    engine: Box<dyn SandboxEngine>,
}

impl Sandbox {
    pub fn new(engine: Box<dyn SandboxEngine>) -> Self {
        Self { engine }
    }

    pub async fn init(&mut self, listener: Option<SandboxListener>) -> Result<()> {
        self.engine
            .init(listener.unwrap_or_default())
            .await
    }

    pub async fn destroy(&mut self) -> Result<()> {
        self.engine
            .destroy()
            .await
    }

    pub async fn exec(&mut self, cmd: &str) -> Result<()> {
        self.engine
            .exec(cmd)
            .await
    }

    pub async fn mount(&mut self, mount: SandboxMount) -> Result<()> {
        self.engine
            .mount(mount)
            .await
    }
}
