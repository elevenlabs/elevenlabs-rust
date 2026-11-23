pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Interruption {
    pub interruption_event: InterruptionEvent,
}

impl Interruption {
    pub fn builder() -> InterruptionBuilder {
        <InterruptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InterruptionBuilder {
    interruption_event: Option<InterruptionEvent>,
}

impl InterruptionBuilder {
    pub fn interruption_event(mut self, value: InterruptionEvent) -> Self {
        self.interruption_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Interruption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`interruption_event`](InterruptionBuilder::interruption_event)
    pub fn build(self) -> Result<Interruption, BuildError> {
        Ok(Interruption {
            interruption_event: self
                .interruption_event
                .ok_or_else(|| BuildError::missing_field("interruption_event"))?,
        })
    }
}
