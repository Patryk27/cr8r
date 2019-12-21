use lib_interop::protocol::for_client::PHelloReply;

pub fn hello() -> PHelloReply {
    PHelloReply {
        version: "0.1.0".into(),
        uptime: 0, // @todo
    }
}