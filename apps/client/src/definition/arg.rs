use anyhow::*;
use structopt::StructOpt;

use lib_interop::domain::DDefinition;

use crate::app::AppContext;

pub use self::{
    dependency::*,
    toolchain::*,
};

mod dependency;
mod toolchain;

#[derive(Debug, StructOpt)]
pub struct DefinitionArg {
    #[structopt(long = "toolchain", short = "t")]
    pub toolchain: Option<ToolchainArg>,

    #[structopt(long = "dependency", short = "d")]
    pub dependencies: Vec<DependencyArg>,
}

impl DefinitionArg {
    pub async fn build(self, ctxt: &mut AppContext) -> Result<DDefinition> {
        unimplemented!()
    }
}
