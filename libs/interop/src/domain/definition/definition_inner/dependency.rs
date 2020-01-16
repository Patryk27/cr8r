use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DAttachmentId, DomainError, DomainResult};
use crate::proto::core::p_definition::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DDependency {
    pub registry: String,
    pub name: String,
    pub action: DDependencyAction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DDependencyAction {
    OverrideUsingAttachment {
        attachment_id: DAttachmentId,
    },

    OverrideUsingVersion {
        version: String,
    },
}

impl TryFrom<PDependency> for DDependency {
    type Error = DomainError;

    fn try_from(PDependency { registry, name, action }: PDependency) -> DomainResult<Self> {
        use p_dependency::*;

        let action = match convert!(action?) {
            Action::OverrideUsingAttachment(POverrideUsingAttachment { attachment_id }) => {
                DDependencyAction::OverrideUsingAttachment {
                    attachment_id: convert!(attachment_id as _),
                }
            }

            Action::OverrideUsingVersion(POverrideUsingVersion { version }) => {
                DDependencyAction::OverrideUsingVersion { version }
            }
        };

        Ok(Self { registry, name, action })
    }
}

impl Into<PDependency> for DDependency {
    fn into(self) -> PDependency {
        use p_dependency::*;

        let Self { registry, name, action } = self;

        let action = Some(match action {
            DDependencyAction::OverrideUsingAttachment { attachment_id } => {
                Action::OverrideUsingAttachment(POverrideUsingAttachment {
                    attachment_id: convert!(attachment_id as _),
                })
            }

            DDependencyAction::OverrideUsingVersion { version } => {
                Action::OverrideUsingVersion(POverrideUsingVersion { version })
            }
        });

        PDependency { registry, name, action }
    }
}