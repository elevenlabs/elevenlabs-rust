pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BackupLlmOverride {
    #[serde(default)]
    pub order: Vec<Llm>,
}

impl BackupLlmOverride {
    pub fn builder() -> BackupLlmOverrideBuilder {
        <BackupLlmOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BackupLlmOverrideBuilder {
    order: Option<Vec<Llm>>,
}

impl BackupLlmOverrideBuilder {
    pub fn order(mut self, value: Vec<Llm>) -> Self {
        self.order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BackupLlmOverride`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order`](BackupLlmOverrideBuilder::order)
    pub fn build(self) -> Result<BackupLlmOverride, BuildError> {
        Ok(BackupLlmOverride {
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
        })
    }
}
