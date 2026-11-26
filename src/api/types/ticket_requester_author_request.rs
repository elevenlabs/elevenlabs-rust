pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TicketRequesterAuthorRequest {}

impl TicketRequesterAuthorRequest {
    pub fn builder() -> TicketRequesterAuthorRequestBuilder {
        <TicketRequesterAuthorRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketRequesterAuthorRequestBuilder {}

impl TicketRequesterAuthorRequestBuilder {
    /// Consumes the builder and constructs a [`TicketRequesterAuthorRequest`].
    pub fn build(self) -> Result<TicketRequesterAuthorRequest, BuildError> {
        Ok(TicketRequesterAuthorRequest {})
    }
}
