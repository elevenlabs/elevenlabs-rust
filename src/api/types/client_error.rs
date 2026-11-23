pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ClientError {
    pub error_event: ErrorEvent,
}

impl ClientError {
    pub fn builder() -> ClientErrorBuilder {
        <ClientErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientErrorBuilder {
    error_event: Option<ErrorEvent>,
}

impl ClientErrorBuilder {
    pub fn error_event(mut self, value: ErrorEvent) -> Self {
        self.error_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error_event`](ClientErrorBuilder::error_event)
    pub fn build(self) -> Result<ClientError, BuildError> {
        Ok(ClientError {
            error_event: self
                .error_event
                .ok_or_else(|| BuildError::missing_field("error_event"))?,
        })
    }
}
