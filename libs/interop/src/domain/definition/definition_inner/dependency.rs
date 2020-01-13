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
    Override {
        version: String,
    },

    Patch {
        source_attachment_id: DAttachmentId,
    },
}

impl TryFrom<PDependency> for DDependency {
    type Error = DomainError;

    fn try_from(PDependency { registry, name, action }: PDependency) -> DomainResult<Self> {
        use p_dependency::*;

        let action = match convert!(action?) {
            Action::Override(POverride { version }) => {
                DDependencyAction::Override { version }
            }

            Action::Patch(PPatch { source_attachment_id }) => {
                DDependencyAction::Patch {
                    source_attachment_id: convert!(source_attachment_id as _),
                }
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
            DDependencyAction::Override { version } => {
                Action::Override(POverride { version })
            }

            DDependencyAction::Patch { source_attachment_id } => {
                Action::Patch(PPatch {
                    source_attachment_id: convert!(source_attachment_id as _),
                })
            }
        });

        PDependency { registry, name, action }
    }
}