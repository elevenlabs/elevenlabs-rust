pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransferToAgentToolConfigInput {
    #[serde(default)]
    pub transfers: Vec<AgentTransferInput>,
}

impl TransferToAgentToolConfigInput {
    pub fn builder() -> TransferToAgentToolConfigInputBuilder {
        <TransferToAgentToolConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferToAgentToolConfigInputBuilder {
    transfers: Option<Vec<AgentTransferInput>>,
}

impl TransferToAgentToolConfigInputBuilder {
    pub fn transfers(mut self, value: Vec<AgentTransferInput>) -> Self {
        self.transfers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferToAgentToolConfigInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfers`](TransferToAgentToolConfigInputBuilder::transfers)
    pub fn build(self) -> Result<TransferToAgentToolConfigInput, BuildError> {
        Ok(TransferToAgentToolConfigInput {
            transfers: self
                .transfers
                .ok_or_else(|| BuildError::missing_field("transfers"))?,
        })
    }
}
