use lib_interop::protocol::for_runner::PHelloReply;

pub fn hello() -> PHelloReply {
    PHelloReply {
        version: "0.1.0".into(),
    }
}