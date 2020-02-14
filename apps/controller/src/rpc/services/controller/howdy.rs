use lib_interop::proto::services::PHowdyReply;

pub fn howdy() -> PHowdyReply {
    PHowdyReply {
        version: "0.1.0".into(),
        uptime: 0, // @todo
    }
}