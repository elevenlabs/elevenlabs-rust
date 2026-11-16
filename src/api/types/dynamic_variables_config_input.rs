pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DynamicVariablesConfigInput(pub serde_json::Value);
