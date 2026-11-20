pub use crate::prelude::*;

/// Unix timestamp (seconds since epoch)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct UnixTimestamp(pub i64);
