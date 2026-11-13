pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BackupLlmDefault {}

impl BackupLlmDefault {
    pub fn builder() -> BackupLlmDefaultBuilder {
        <BackupLlmDefaultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BackupLlmDefaultBuilder {}

impl BackupLlmDefaultBuilder {
    /// Consumes the builder and constructs a [`BackupLlmDefault`].
    pub fn build(self) -> Result<BackupLlmDefault, BuildError> {
        Ok(BackupLlmDefault {})
    }
}
