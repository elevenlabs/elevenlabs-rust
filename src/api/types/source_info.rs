pub use crate::prelude::*;

/// Information about the source of conversation initiation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourceInfo {
    /// Source of the conversation initiation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceInfoSource>,
    /// The SDK version number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl SourceInfo {
    pub fn builder() -> SourceInfoBuilder {
        <SourceInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceInfoBuilder {
    source: Option<SourceInfoSource>,
    version: Option<String>,
}

impl SourceInfoBuilder {
    pub fn source(mut self, value: SourceInfoSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SourceInfo`].
    pub fn build(self) -> Result<SourceInfo, BuildError> {
        Ok(SourceInfo {
            source: self.source,
            version: self.version,
        })
    }
}
