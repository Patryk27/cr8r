use std::path::PathBuf;

use crate::Result;

pub struct Client {
    path: PathBuf,
}

impl Client {
    pub fn new() -> Self {
        Self {
            // @todo auto-detect
            path: PathBuf::from("/snap/bin/lxc"),
        }
    }

    pub fn launch(&mut self, container: &str, image: &str) -> Result<()> {
        self.run(&[
            "launch",
            image,
            container,
            "-c",
            "security.nesting=true",
        ])
    }

    pub fn delete(&mut self, _name: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn list(&mut self, _name: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn exec(&mut self, _container: &str, _command: &str) -> Result<()> {
        unimplemented!()
    }

    fn run(&mut self, _args: &[&str]) -> Result<()> {
        unimplemented!()
    }
}