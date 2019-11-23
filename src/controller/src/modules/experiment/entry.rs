use actix::Addr;

use lib_protocol as proto;

use crate::modules::Experiment;

pub struct ExperimentEntry {
    pub actor: Addr<Experiment>,
    pub status: proto::ExperimentStatus,
    pub definition: proto::ExperimentDefinition,
}