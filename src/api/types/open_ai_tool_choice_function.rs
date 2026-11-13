pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OpenAiToolChoiceFunction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<OpenAiFunctionToolType>,
    pub function: OpenAiToolChoiceFunctionInner,
}

impl OpenAiToolChoiceFunction {
    pub fn builder() -> OpenAiToolChoiceFunctionBuilder {
        <OpenAiToolChoiceFunctionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiToolChoiceFunctionBuilder {
    r#type: Option<OpenAiFunctionToolType>,
    function: Option<OpenAiToolChoiceFunctionInner>,
}

impl OpenAiToolChoiceFunctionBuilder {
    pub fn r#type(mut self, value: OpenAiFunctionToolType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn function(mut self, value: OpenAiToolChoiceFunctionInner) -> Self {
        self.function = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiToolChoiceFunction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`function`](OpenAiToolChoiceFunctionBuilder::function)
    pub fn build(self) -> Result<OpenAiToolChoiceFunction, BuildError> {
        Ok(OpenAiToolChoiceFunction {
            r#type: self.r#type,
            function: self
                .function
                .ok_or_else(|| BuildError::missing_field("function"))?,
        })
    }
}
