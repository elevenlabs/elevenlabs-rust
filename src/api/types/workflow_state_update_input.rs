pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStateUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AstDynamicVariableNodeInputType>,
    /// Name of the dynamic variable to assign.
    #[serde(default)]
    pub variable_name: String,
    pub expression: AstNodeInput,
    /// When false, the LLM may omit the values this update relies on. The update is then skipped and the variable keeps its current value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl WorkflowStateUpdateInput {
    pub fn builder() -> WorkflowStateUpdateInputBuilder {
        <WorkflowStateUpdateInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowStateUpdateInputBuilder {
    r#type: Option<AstDynamicVariableNodeInputType>,
    variable_name: Option<String>,
    expression: Option<AstNodeInput>,
    required: Option<bool>,
}

impl WorkflowStateUpdateInputBuilder {
    pub fn r#type(mut self, value: AstDynamicVariableNodeInputType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn variable_name(mut self, value: impl Into<String>) -> Self {
        self.variable_name = Some(value.into());
        self
    }

    pub fn expression(mut self, value: AstNodeInput) -> Self {
        self.expression = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowStateUpdateInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`variable_name`](WorkflowStateUpdateInputBuilder::variable_name)
    /// - [`expression`](WorkflowStateUpdateInputBuilder::expression)
    pub fn build(self) -> Result<WorkflowStateUpdateInput, BuildError> {
        Ok(WorkflowStateUpdateInput {
            r#type: self.r#type,
            variable_name: self
                .variable_name
                .ok_or_else(|| BuildError::missing_field("variable_name"))?,
            expression: self
                .expression
                .ok_or_else(|| BuildError::missing_field("expression"))?,
            required: self.required,
        })
    }
}
