pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketRequesterByEmailRequest {
    #[serde(default)]
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}

impl TicketRequesterByEmailRequest {
    pub fn builder() -> TicketRequesterByEmailRequestBuilder {
        <TicketRequesterByEmailRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketRequesterByEmailRequestBuilder {
    value: Option<String>,
    full_name: Option<String>,
}

impl TicketRequesterByEmailRequestBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.full_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TicketRequesterByEmailRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](TicketRequesterByEmailRequestBuilder::value)
    pub fn build(self) -> Result<TicketRequesterByEmailRequest, BuildError> {
        Ok(TicketRequesterByEmailRequest {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
            full_name: self.full_name,
        })
    }
}
