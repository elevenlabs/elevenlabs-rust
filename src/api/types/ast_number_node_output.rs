pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstNumberNodeOutput {
    /// Value of this literal.
    #[serde(default)]
    pub value: AstNumberNodeOutputValue,
}

impl AstNumberNodeOutput {
    pub fn builder() -> AstNumberNodeOutputBuilder {
        <AstNumberNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstNumberNodeOutputBuilder {
    value: Option<AstNumberNodeOutputValue>,
}

impl AstNumberNodeOutputBuilder {
    pub fn value(mut self, value: AstNumberNodeOutputValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstNumberNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](AstNumberNodeOutputBuilder::value)
    pub fn build(self) -> Result<AstNumberNodeOutput, BuildError> {
        Ok(AstNumberNodeOutput {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
