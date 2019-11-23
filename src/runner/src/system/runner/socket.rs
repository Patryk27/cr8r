use actix::io::SinkWrite;
use actix_codec::Framed;
use actix_web_actors::ws;
use awc::BoxedSocket;
use awc::ws::Codec;
use futures::stream::SplitSink;
use log::*;

use lib_protocol as proto;

pub type RunnerSocketSink = SinkWrite<SplitSink<Framed<BoxedSocket, Codec>>>;

pub struct RunnerSocket {
    sink: RunnerSocketSink,
}

impl RunnerSocket {
    pub fn new(sink: RunnerSocketSink) -> Self {
        Self { sink }
    }

    pub fn send(&mut self, msg: proto::runner::Request) {
        debug!("Sending message: {:?}", msg);

        self.sink
            .write(ws::Message::Text(msg.marshal()))
            .unwrap();
    }

    pub fn recv(&self, msg: ws::Frame) -> Option<proto::runner::Response> {
        if let ws::Frame::Text(Some(msg)) = msg {
            Some(proto::runner::Response::unmarshal(msg))
        } else {
            None
        }
    }
}