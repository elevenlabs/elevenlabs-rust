pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AstStringNodeInput {
    /// Value of this literal.
    #[serde(default)]
    pub value: String,
}

impl AstStringNodeInput {
    pub fn builder() -> AstStringNodeInputBuilder {
        <AstStringNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstStringNodeInputBuilder {
    value: Option<String>,
}

impl AstStringNodeInputBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AstStringNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](AstStringNodeInputBuilder::value)
    pub fn build(self) -> Result<AstStringNodeInput, BuildError> {
        Ok(AstStringNodeInput {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
