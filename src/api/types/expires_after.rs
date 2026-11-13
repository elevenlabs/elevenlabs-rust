pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExpiresAfter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<Anchor>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub seconds: i64,
}

impl ExpiresAfter {
    pub fn builder() -> ExpiresAfterBuilder {
        <ExpiresAfterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpiresAfterBuilder {
    anchor: Option<Anchor>,
    seconds: Option<i64>,
}

impl ExpiresAfterBuilder {
    pub fn anchor(mut self, value: Anchor) -> Self {
        self.anchor = Some(value);
        self
    }

    pub fn seconds(mut self, value: i64) -> Self {
        self.seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExpiresAfter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`seconds`](ExpiresAfterBuilder::seconds)
    pub fn build(self) -> Result<ExpiresAfter, BuildError> {
        Ok(ExpiresAfter {
            anchor: self.anchor,
            seconds: self
                .seconds
                .ok_or_else(|| BuildError::missing_field("seconds"))?,
        })
    }
}
