pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClientToolCall {
    pub client_tool_call: ClientToolCallClientToolCall,
}

impl ClientToolCall {
    pub fn builder() -> ClientToolCallBuilder {
        <ClientToolCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientToolCallBuilder {
    client_tool_call: Option<ClientToolCallClientToolCall>,
}

impl ClientToolCallBuilder {
    pub fn client_tool_call(mut self, value: ClientToolCallClientToolCall) -> Self {
        self.client_tool_call = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientToolCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_tool_call`](ClientToolCallBuilder::client_tool_call)
    pub fn build(self) -> Result<ClientToolCall, BuildError> {
        Ok(ClientToolCall {
            client_tool_call: self
                .client_tool_call
                .ok_or_else(|| BuildError::missing_field("client_tool_call"))?,
        })
    }
}
