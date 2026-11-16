pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstConditionalOperatorNodeInput {
    pub condition: Box<AstNodeInput>,
    #[serde(rename = "trueExpression")]
    pub true_expression: Box<AstNodeInput>,
    #[serde(rename = "falseExpression")]
    pub false_expression: Box<AstNodeInput>,
}

impl AstConditionalOperatorNodeInput {
    pub fn builder() -> AstConditionalOperatorNodeInputBuilder {
        <AstConditionalOperatorNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstConditionalOperatorNodeInputBuilder {
    condition: Option<Box<AstNodeInput>>,
    true_expression: Option<Box<AstNodeInput>>,
    false_expression: Option<Box<AstNodeInput>>,
}

impl AstConditionalOperatorNodeInputBuilder {
    pub fn condition(mut self, value: Box<AstNodeInput>) -> Self {
        self.condition = Some(value);
        self
    }

    pub fn true_expression(mut self, value: Box<AstNodeInput>) -> Self {
        self.true_expression = Some(value);
        self
    }

    pub fn false_expression(mut self, value: Box<AstNodeInput>) -> Self {
        self.false_expression = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstConditionalOperatorNodeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`condition`](AstConditionalOperatorNodeInputBuilder::condition)
    /// - [`true_expression`](AstConditionalOperatorNodeInputBuilder::true_expression)
    /// - [`false_expression`](AstConditionalOperatorNodeInputBuilder::false_expression)
    pub fn build(self) -> Result<AstConditionalOperatorNodeInput, BuildError> {
        Ok(AstConditionalOperatorNodeInput {
            condition: self
                .condition
                .ok_or_else(|| BuildError::missing_field("condition"))?,
            true_expression: self
                .true_expression
                .ok_or_else(|| BuildError::missing_field("true_expression"))?,
            false_expression: self
                .false_expression
                .ok_or_else(|| BuildError::missing_field("false_expression"))?,
        })
    }
}
