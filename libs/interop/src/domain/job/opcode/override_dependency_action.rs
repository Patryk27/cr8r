use std::convert::TryFrom;

use crate::convert;
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
        use crate::proto::core::p_job_opcode::p_override_dependency::*;

        match value {
            Action::UseAttachmentId(attachment_id) => {
                Ok(DOverrideDependencyAction::UseAttachment {
                    attachment_id: convert!(attachment_id as _),
                })
            }

            Action::UseVersion(version) => {
                Ok(DOverrideDependencyAction::UseVersion { version })
            }
        }
    }
}

impl Into<Action> for DOverrideDependencyAction {
    fn into(self) -> Action {
        use crate::proto::core::p_job_opcode::p_override_dependency::*;

        match self {
            DOverrideDependencyAction::UseAttachment { attachment_id } => {
                Action::UseAttachmentId(
                    convert!(attachment_id as _),
                )
            }

            DOverrideDependencyAction::UseVersion { version } => {
                Action::UseVersion(version)
            }
        }
    }
}