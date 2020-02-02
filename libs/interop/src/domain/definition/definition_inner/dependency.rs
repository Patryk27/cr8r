use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DAttachmentId, DomainError, DomainResult};
use crate::proto::core::p_definition::*;
use crate::proto::core::p_definition::p_dependency_def::PDependencyDefSource;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DDependencyDef {
    pub name: String,
    pub source: DDependencyDefSource,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DDependencyDefSource {
    Branch {
        branch: String,
    },

    Tag {
        tag: String,
    },

    Version {
        version: String,
    },

    Path {
        attachment_id: DAttachmentId,
    },
}

impl TryFrom<PDependencyDef> for DDependencyDef {
    type Error = DomainError;

    fn try_from(PDependencyDef { name, source }: PDependencyDef) -> DomainResult<Self> {
        use p_dependency_def::p_dependency_def_source::*;

        let source_ty = convert!(source?).ty;

        let source = match convert!(source_ty?) {
            Ty::Branch(PBranchSource { branch }) => {
                DDependencyDefSource::Branch { branch }
            }

            Ty::Tag(PTagSource { tag }) => {
                DDependencyDefSource::Tag { tag }
            }

            Ty::Version(PVersionSource { version }) => {
                DDependencyDefSource::Version { version }
            }

            Ty::Path(PPathSource { attachment_id }) => {
                DDependencyDefSource::Path {
                    attachment_id: convert!(attachment_id as _),
                }
            }
        };

        Ok(Self { name, source })
    }
}

impl Into<PDependencyDef> for DDependencyDef {
    fn into(self) -> PDependencyDef {
        use p_dependency_def::p_dependency_def_source::*;

        let Self { name, source } = self;

        let source_ty = Some(match source {
            DDependencyDefSource::Branch { branch } => {
                Ty::Branch(PBranchSource { branch })
            }

            DDependencyDefSource::Tag { tag } => {
                Ty::Tag(PTagSource { tag })
            }

            DDependencyDefSource::Version { version } => {
                Ty::Version(PVersionSource { version })
            }

            DDependencyDefSource::Path { attachment_id } => {
                Ty::Path(PPathSource {
                    attachment_id: convert!(attachment_id as _),
                })
            }
        });

        let source = Some(PDependencyDefSource {
            ty: source_ty,
        });

        PDependencyDef { name, source }
    }
}