pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketRequesterByUserIdRequest {
    #[serde(default)]
    pub value: String,
}

impl TicketRequesterByUserIdRequest {
    pub fn builder() -> TicketRequesterByUserIdRequestBuilder {
        <TicketRequesterByUserIdRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketRequesterByUserIdRequestBuilder {
    value: Option<String>,
}

impl TicketRequesterByUserIdRequestBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TicketRequesterByUserIdRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](TicketRequesterByUserIdRequestBuilder::value)
    pub fn build(self) -> Result<TicketRequesterByUserIdRequest, BuildError> {
        Ok(TicketRequesterByUserIdRequest {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
