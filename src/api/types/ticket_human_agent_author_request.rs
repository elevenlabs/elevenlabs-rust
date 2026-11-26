pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TicketHumanAgentAuthorRequest {}

impl TicketHumanAgentAuthorRequest {
    pub fn builder() -> TicketHumanAgentAuthorRequestBuilder {
        <TicketHumanAgentAuthorRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketHumanAgentAuthorRequestBuilder {}

impl TicketHumanAgentAuthorRequestBuilder {
    /// Consumes the builder and constructs a [`TicketHumanAgentAuthorRequest`].
    pub fn build(self) -> Result<TicketHumanAgentAuthorRequest, BuildError> {
        Ok(TicketHumanAgentAuthorRequest {})
    }
}
