use serde::Deserialize;

pub use self::{
    environment::*,
    fauna::*,
    flora::*,
};

mod environment;
mod flora;
mod fauna;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ecosystem {
    pub environment: Environment,
    pub flora: Flora,
    pub fauna: Fauna,
}