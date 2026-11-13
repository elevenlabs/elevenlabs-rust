pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransferToAgentToolResultErrorModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DubbingErrorPayloadMessageType>,
    #[serde(default)]
    pub from_agent: String,
    #[serde(default)]
    pub error: String,
}

impl TransferToAgentToolResultErrorModel {
    pub fn builder() -> TransferToAgentToolResultErrorModelBuilder {
        <TransferToAgentToolResultErrorModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferToAgentToolResultErrorModelBuilder {
    status: Option<DubbingErrorPayloadMessageType>,
    from_agent: Option<String>,
    error: Option<String>,
}

impl TransferToAgentToolResultErrorModelBuilder {
    pub fn status(mut self, value: DubbingErrorPayloadMessageType) -> Self {
        self.status = Some(value);
        self
    }

    pub fn from_agent(mut self, value: impl Into<String>) -> Self {
        self.from_agent = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferToAgentToolResultErrorModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_agent`](TransferToAgentToolResultErrorModelBuilder::from_agent)
    /// - [`error`](TransferToAgentToolResultErrorModelBuilder::error)
    pub fn build(self) -> Result<TransferToAgentToolResultErrorModel, BuildError> {
        Ok(TransferToAgentToolResultErrorModel {
            status: self.status,
            from_agent: self
                .from_agent
                .ok_or_else(|| BuildError::missing_field("from_agent"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
