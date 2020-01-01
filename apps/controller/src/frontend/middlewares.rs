use std::error::Error;

use hyper::{Body, Request, Response};
use tonic::body::BoxBody;
use tonic::codegen::Service;

pub use self::authorize::*;

type MiddlewareRequest = Request<Body>;
type MiddlewareResponse = Result<Response<BoxBody>, Box<dyn Error + Send + Sync>>;

mod authorize;