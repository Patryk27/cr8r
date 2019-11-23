use actix::Addr;

use crate::modules::System;

#[derive(Clone)]
pub struct HttpState {
    pub system: Addr<System>,
}