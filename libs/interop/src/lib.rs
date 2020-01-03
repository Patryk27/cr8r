#![feature(type_ascription)]

pub use self::error::*;

mod error;

pub mod client;
pub mod domain;
pub mod proto;