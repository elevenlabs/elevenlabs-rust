pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ContextUsage {
    pub context_usage_event: ContextUsageEvent,
}

impl ContextUsage {
    pub fn builder() -> ContextUsageBuilder {
        <ContextUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContextUsageBuilder {
    context_usage_event: Option<ContextUsageEvent>,
}

impl ContextUsageBuilder {
    pub fn context_usage_event(mut self, value: ContextUsageEvent) -> Self {
        self.context_usage_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContextUsage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context_usage_event`](ContextUsageBuilder::context_usage_event)
    pub fn build(self) -> Result<ContextUsage, BuildError> {
        Ok(ContextUsage {
            context_usage_event: self
                .context_usage_event
                .ok_or_else(|| BuildError::missing_field("context_usage_event"))?,
        })
    }
}
