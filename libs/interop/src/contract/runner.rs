use crate::protocol::core::PRunner;

pub use self::{
    id::*,
    name::*,
    status::*,
};

mod id;
mod name;
mod status;

#[derive(Clone, Debug)]
pub struct CRunner {
    //
}

impl Into<PRunner> for CRunner {
    fn into(self) -> PRunner {
        unimplemented!()
    }
}