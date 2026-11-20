pub use crate::prelude::*;

/// `ClientToolCallClientEvent` as a stream event, because its custom `__init__` can't parse its own JSON.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClientToolCallEvent {
    pub client_tool_call: ClientToolCallClientEventData,
}

impl ClientToolCallEvent {
    pub fn builder() -> ClientToolCallEventBuilder {
        <ClientToolCallEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientToolCallEventBuilder {
    client_tool_call: Option<ClientToolCallClientEventData>,
}

impl ClientToolCallEventBuilder {
    pub fn client_tool_call(mut self, value: ClientToolCallClientEventData) -> Self {
        self.client_tool_call = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientToolCallEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_tool_call`](ClientToolCallEventBuilder::client_tool_call)
    pub fn build(self) -> Result<ClientToolCallEvent, BuildError> {
        Ok(ClientToolCallEvent {
            client_tool_call: self
                .client_tool_call
                .ok_or_else(|| BuildError::missing_field("client_tool_call"))?,
        })
    }
}
