pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AstBooleanNodeOutput {
    /// Value of this literal.
    #[serde(default)]
    pub value: bool,
}

impl AstBooleanNodeOutput {
    pub fn builder() -> AstBooleanNodeOutputBuilder {
        <AstBooleanNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstBooleanNodeOutputBuilder {
    value: Option<bool>,
}

impl AstBooleanNodeOutputBuilder {
    pub fn value(mut self, value: bool) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstBooleanNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](AstBooleanNodeOutputBuilder::value)
    pub fn build(self) -> Result<AstBooleanNodeOutput, BuildError> {
        Ok(AstBooleanNodeOutput {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
