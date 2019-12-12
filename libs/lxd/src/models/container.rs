use serde::Deserialize;

use crate::LxdContainerName;

#[derive(Clone, Debug, Deserialize)]
pub struct LxdContainer {
    pub name: LxdContainerName,
}