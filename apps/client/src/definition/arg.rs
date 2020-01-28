use anyhow::*;
use structopt::StructOpt;

use lib_interop::domain::DDefinition;

use self::parsers::*;

mod parsers;

#[derive(Debug, StructOpt)]
pub struct DefinitionArg {
    #[structopt(long = "toolchain", short = "t")]
    toolchain: Option<String>,

    #[structopt(long = "dependency", short = "d")]
    dependencies: Vec<String>,
}

impl DefinitionArg {
    pub fn parse(self) -> Result<DDefinition> {
        Ok(DDefinition {
            toolchain: parse_toolchain(self.toolchain),
            dependencies: parse_dependencies(self.dependencies)?,
        })
    }
}
