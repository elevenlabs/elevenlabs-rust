pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstNumberNodeInput {
    /// Value of this literal.
    #[serde(default)]
    pub value: AstNumberNodeInputValue,
}

impl AstNumberNodeInput {
    pub fn builder() -> AstNumberNodeInputBuilder {
        <AstNumberNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstNumberNodeInputBuilder {
    value: Option<AstNumberNodeInputValue>,
}

impl AstNumberNodeInputBuilder {
    pub fn value(mut self, value: AstNumberNodeInputValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstNumberNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](AstNumberNodeInputBuilder::value)
    pub fn build(self) -> Result<AstNumberNodeInput, BuildError> {
        Ok(AstNumberNodeInput {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
