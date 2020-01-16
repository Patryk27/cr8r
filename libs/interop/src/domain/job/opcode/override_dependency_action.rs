use std::convert::TryFrom;

use crate::domain::{DAttachmentId, DomainError, DomainResult};
use crate::proto::core::p_job_opcode::p_override_dependency::Action;

#[derive(Clone, Debug)]
pub enum DOverrideDependencyAction {
    UseAttachment {
        attachment_id: DAttachmentId,
    },

    UseVersion {
        version: String,
    },
}

impl TryFrom<Action> for DOverrideDependencyAction {
    type Error = DomainError;

    fn try_from(value: Action) -> DomainResult<Self> {
        unimplemented!()
    }
}

impl Into<Action> for DOverrideDependencyAction {
    fn into(self) -> Action {
        unimplemented!()
    }
}