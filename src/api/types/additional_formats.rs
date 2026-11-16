pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AdditionalFormats(pub Vec<ExportOptions>);
