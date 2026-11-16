pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BackupLlmDisabled {}

impl BackupLlmDisabled {
    pub fn builder() -> BackupLlmDisabledBuilder {
        <BackupLlmDisabledBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BackupLlmDisabledBuilder {}

impl BackupLlmDisabledBuilder {
    /// Consumes the builder and constructs a [`BackupLlmDisabled`].
    pub fn build(self) -> Result<BackupLlmDisabled, BuildError> {
        Ok(BackupLlmDisabled {})
    }
}
