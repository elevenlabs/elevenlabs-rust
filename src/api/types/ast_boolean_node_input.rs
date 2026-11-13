pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AstBooleanNodeInput {
    /// Value of this literal.
    #[serde(default)]
    pub value: bool,
}

impl AstBooleanNodeInput {
    pub fn builder() -> AstBooleanNodeInputBuilder {
        <AstBooleanNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstBooleanNodeInputBuilder {
    value: Option<bool>,
}

impl AstBooleanNodeInputBuilder {
    pub fn value(mut self, value: bool) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstBooleanNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](AstBooleanNodeInputBuilder::value)
    pub fn build(self) -> Result<AstBooleanNodeInput, BuildError> {
        Ok(AstBooleanNodeInput {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
