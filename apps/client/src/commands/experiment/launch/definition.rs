use std::convert::TryInto;

use anyhow::Result;
use structopt::StructOpt;

use lib_interop::domain::DDefinition;

#[derive(Debug, StructOpt)]
pub struct Definition {
    #[structopt(long = "toolchain")]
    toolchain: String,

    #[structopt(long = "crate")]
    krates: Vec<String>,
}

impl Definition {
    pub fn parse(self) -> Result<DDefinition> {
        unimplemented!()
    }
}
