use actix::{Actor, StreamHandler};
use actix::dev::Stream;
use actix::io::SinkWrite;
use actix_codec::Framed;
use awc::BoxedSocket;
use awc::ws::Codec;

pub use self::{
    actor::*,
    socket::*,
    state::*,
};

mod actor;
mod socket;
mod state;

pub struct Runner;

impl Runner {
    pub fn spawn(runner_name: String, controller_secret: String, stream: Framed<BoxedSocket, Codec>) {
        let (sink, stream) = stream.split();

        RunnerActor::create(|ctx| {
            let sink = SinkWrite::new(sink, ctx);

            RunnerActor::add_stream(stream, ctx);

            RunnerActor::new(
                runner_name,
                controller_secret,
                RunnerSocket::new(sink),
            )
        });
    }
}