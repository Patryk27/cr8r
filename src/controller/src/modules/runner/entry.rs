use actix::Addr;

use lib_protocol as proto;

use crate::modules::Runner;

pub struct RunnerEntry {
    pub actor: Addr<Runner>,
    pub name: proto::RunnerName,
    pub status: proto::RunnerStatus,
}