use std::collections::HashMap;

use serde::Deserialize;

pub type Fauna = HashMap<FaunaProviderName, FaunaProvider>;
pub type FaunaProviderName = String;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FaunaProvider {
    pub requires: Vec<String>,
    pub provides: String,
}