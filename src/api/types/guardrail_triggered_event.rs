pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GuardrailTriggeredEvent {
    #[serde(default)]
    pub guardrail_name: String,
}

impl GuardrailTriggeredEvent {
    pub fn builder() -> GuardrailTriggeredEventBuilder {
        <GuardrailTriggeredEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GuardrailTriggeredEventBuilder {
    guardrail_name: Option<String>,
}

impl GuardrailTriggeredEventBuilder {
    pub fn guardrail_name(mut self, value: impl Into<String>) -> Self {
        self.guardrail_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GuardrailTriggeredEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`guardrail_name`](GuardrailTriggeredEventBuilder::guardrail_name)
    pub fn build(self) -> Result<GuardrailTriggeredEvent, BuildError> {
        Ok(GuardrailTriggeredEvent {
            guardrail_name: self
                .guardrail_name
                .ok_or_else(|| BuildError::missing_field("guardrail_name"))?,
        })
    }
}
