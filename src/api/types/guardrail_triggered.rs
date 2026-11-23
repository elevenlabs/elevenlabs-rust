pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GuardrailTriggered {
    pub guardrail_triggered_event: GuardrailTriggeredEvent,
}

impl GuardrailTriggered {
    pub fn builder() -> GuardrailTriggeredBuilder {
        <GuardrailTriggeredBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GuardrailTriggeredBuilder {
    guardrail_triggered_event: Option<GuardrailTriggeredEvent>,
}

impl GuardrailTriggeredBuilder {
    pub fn guardrail_triggered_event(mut self, value: GuardrailTriggeredEvent) -> Self {
        self.guardrail_triggered_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GuardrailTriggered`].
    /// This method will fail if any of the following fields are not set:
    /// - [`guardrail_triggered_event`](GuardrailTriggeredBuilder::guardrail_triggered_event)
    pub fn build(self) -> Result<GuardrailTriggered, BuildError> {
        Ok(GuardrailTriggered {
            guardrail_triggered_event: self
                .guardrail_triggered_event
                .ok_or_else(|| BuildError::missing_field("guardrail_triggered_event"))?,
        })
    }
}
