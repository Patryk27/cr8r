use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use crate::conv;
use crate::models::{ModelError, ModelResult};
use crate::proto::models::{PAttachment, PAttachmentSize};

pub use self::{
    id::*,
    name::*,
};

mod id;
mod name;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DAttachment {
    pub id: DAttachmentId,
    pub name: DAttachmentName,
    pub size: PAttachmentSize,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<PAttachment> for DAttachment {
    type Error = ModelError;

    fn try_from(PAttachment { id, name, size, created_at }: PAttachment) -> ModelResult<Self> {
        Ok(Self {
            id: conv!(id as _),
            name: conv!(name as _),
            size,
            created_at: conv!(created_at as DateTime),
        })
    }
}

impl Into<PAttachment> for DAttachment {
    fn into(self) -> PAttachment {
        let Self { id, name, size, created_at } = self;

        PAttachment {
            id: conv!(id as _),
            name: conv!(name as _),
            size,
            created_at: created_at.to_rfc3339(),
        }
    }
}