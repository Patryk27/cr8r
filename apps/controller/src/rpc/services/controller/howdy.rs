use lib_interop::proto::services::PHowdyReply;

use crate::build;

pub fn howdy() -> PHowdyReply {
    PHowdyReply {
        version: build::PKG_VERSION.into(),
        uptime: 0, // @todo
    }
}