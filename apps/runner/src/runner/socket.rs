use actix::io::SinkWrite;
use actix_codec::Framed;
use actix_web_actors::ws;
use awc::BoxedSocket;
use awc::ws::Codec;
use futures::stream::SplitSink;
use log::*;

use lib_runner_protocol::{ControllerMessage, RunnerMessage};

pub type RunnerSocketSink = SinkWrite<SplitSink<Framed<BoxedSocket, Codec>>>;

pub struct RunnerSocket {
    sink: RunnerSocketSink,
}

impl RunnerSocket {
    pub fn new(sink: RunnerSocketSink) -> Self {
        Self { sink }
    }

    pub fn send(&mut self, msg: ControllerMessage) {
        debug!("Sending message: {:?}", msg);

        self.sink
            .write(ws::Message::Text(msg.marshal()))
            .unwrap();
    }

    pub fn parse(&self, msg: ws::Frame) -> Option<RunnerMessage> {
        if let ws::Frame::Text(Some(msg)) = msg {
            Some(RunnerMessage::unmarshal(&msg))
        } else {
            None
        }
    }
}