use std::convert::TryFrom;

use crate::conv;
use crate::models::{DAttachmentId, ModelError, ModelResult};
use crate::proto::models::p_definition::*;
use crate::proto::models::p_definition::p_dependency_def::PDependencyDefSource;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DDependencyDef {
    pub name: String,
    pub source: DDependencySourceDef,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DDependencySourceDef {
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
    type Error = ModelError;

    fn try_from(PDependencyDef { name, source }: PDependencyDef) -> ModelResult<Self> {
        use p_dependency_def::p_dependency_def_source::*;

        let source_ty = conv!(source?).ty;

        let source = match conv!(source_ty?) {
            Ty::Branch(PBranchSource { branch }) => {
                DDependencySourceDef::Branch { branch }
            }

            Ty::Tag(PTagSource { tag }) => {
                DDependencySourceDef::Tag { tag }
            }

            Ty::Version(PVersionSource { version }) => {
                DDependencySourceDef::Version { version }
            }

            Ty::Path(PPathSource { attachment_id }) => {
                DDependencySourceDef::Path {
                    attachment_id: conv!(attachment_id as _),
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
            DDependencySourceDef::Branch { branch } => {
                Ty::Branch(PBranchSource { branch })
            }

            DDependencySourceDef::Tag { tag } => {
                Ty::Tag(PTagSource { tag })
            }

            DDependencySourceDef::Version { version } => {
                Ty::Version(PVersionSource { version })
            }

            DDependencySourceDef::Path { attachment_id } => {
                Ty::Path(PPathSource {
                    attachment_id: conv!(attachment_id as _),
                })
            }
        });

        let source = Some(PDependencyDefSource {
            ty: source_ty,
        });

        PDependencyDef { name, source }
    }
}