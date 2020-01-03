use lib_interop::proto::controller::PHowdyReply;

pub fn howdy() -> PHowdyReply {
    PHowdyReply {
        version: "0.1.0".into(),
        uptime: 0, // @todo
    }
}