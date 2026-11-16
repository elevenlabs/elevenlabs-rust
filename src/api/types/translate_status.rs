pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateStatus {
    #[serde(default)]
    pub status: String,
}

impl TranslateStatus {
    pub fn builder() -> TranslateStatusBuilder {
        <TranslateStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateStatusBuilder {
    status: Option<String>,
}

impl TranslateStatusBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](TranslateStatusBuilder::status)
    pub fn build(self) -> Result<TranslateStatus, BuildError> {
        Ok(TranslateStatus {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
