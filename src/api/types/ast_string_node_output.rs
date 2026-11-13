pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AstStringNodeOutput {
    /// Value of this literal.
    #[serde(default)]
    pub value: String,
}

impl AstStringNodeOutput {
    pub fn builder() -> AstStringNodeOutputBuilder {
        <AstStringNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstStringNodeOutputBuilder {
    value: Option<String>,
}

impl AstStringNodeOutputBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AstStringNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](AstStringNodeOutputBuilder::value)
    pub fn build(self) -> Result<AstStringNodeOutput, BuildError> {
        Ok(AstStringNodeOutput {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
