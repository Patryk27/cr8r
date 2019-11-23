use serde::Deserialize;

pub use self::{
    fauna::*,
    flora::*,
};

mod flora;
mod fauna;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ecosystem {
    pub flora: Flora,
    pub fauna: Fauna,
}