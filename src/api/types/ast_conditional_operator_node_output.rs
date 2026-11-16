pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstConditionalOperatorNodeOutput {
    pub condition: Box<AstNodeOutput>,
    #[serde(rename = "trueExpression")]
    pub true_expression: Box<AstNodeOutput>,
    #[serde(rename = "falseExpression")]
    pub false_expression: Box<AstNodeOutput>,
}

impl AstConditionalOperatorNodeOutput {
    pub fn builder() -> AstConditionalOperatorNodeOutputBuilder {
        <AstConditionalOperatorNodeOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstConditionalOperatorNodeOutputBuilder {
    condition: Option<Box<AstNodeOutput>>,
    true_expression: Option<Box<AstNodeOutput>>,
    false_expression: Option<Box<AstNodeOutput>>,
}

impl AstConditionalOperatorNodeOutputBuilder {
    pub fn condition(mut self, value: Box<AstNodeOutput>) -> Self {
        self.condition = Some(value);
        self
    }

    pub fn true_expression(mut self, value: Box<AstNodeOutput>) -> Self {
        self.true_expression = Some(value);
        self
    }

    pub fn false_expression(mut self, value: Box<AstNodeOutput>) -> Self {
        self.false_expression = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstConditionalOperatorNodeOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`condition`](AstConditionalOperatorNodeOutputBuilder::condition)
    /// - [`true_expression`](AstConditionalOperatorNodeOutputBuilder::true_expression)
    /// - [`false_expression`](AstConditionalOperatorNodeOutputBuilder::false_expression)
    pub fn build(self) -> Result<AstConditionalOperatorNodeOutput, BuildError> {
        Ok(AstConditionalOperatorNodeOutput {
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
