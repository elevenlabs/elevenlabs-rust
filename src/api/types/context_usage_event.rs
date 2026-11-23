pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ContextUsageEvent {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
    /// Identifier of the LLM whose context window is reported.
    #[serde(default)]
    pub model: String,
    /// Prompt size of the turn's last LLM generation in provider tokens (uncached input plus cache reads and writes).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub context_tokens: i64,
    /// The model's maximum context window size in tokens.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub context_limit_tokens: i64,
}

impl ContextUsageEvent {
    pub fn builder() -> ContextUsageEventBuilder {
        <ContextUsageEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContextUsageEventBuilder {
    event_id: Option<i64>,
    model: Option<String>,
    context_tokens: Option<i64>,
    context_limit_tokens: Option<i64>,
}

impl ContextUsageEventBuilder {
    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn context_tokens(mut self, value: i64) -> Self {
        self.context_tokens = Some(value);
        self
    }

    pub fn context_limit_tokens(mut self, value: i64) -> Self {
        self.context_limit_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContextUsageEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](ContextUsageEventBuilder::event_id)
    /// - [`model`](ContextUsageEventBuilder::model)
    /// - [`context_tokens`](ContextUsageEventBuilder::context_tokens)
    /// - [`context_limit_tokens`](ContextUsageEventBuilder::context_limit_tokens)
    pub fn build(self) -> Result<ContextUsageEvent, BuildError> {
        Ok(ContextUsageEvent {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            context_tokens: self
                .context_tokens
                .ok_or_else(|| BuildError::missing_field("context_tokens"))?,
            context_limit_tokens: self
                .context_limit_tokens
                .ok_or_else(|| BuildError::missing_field("context_limit_tokens"))?,
        })
    }
}
