pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingStatus {
    #[serde(default)]
    pub status: String,
}

impl SubtitlingStatus {
    pub fn builder() -> SubtitlingStatusBuilder {
        <SubtitlingStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingStatusBuilder {
    status: Option<String>,
}

impl SubtitlingStatusBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](SubtitlingStatusBuilder::status)
    pub fn build(self) -> Result<SubtitlingStatus, BuildError> {
        Ok(SubtitlingStatus {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
