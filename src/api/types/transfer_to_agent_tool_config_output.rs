pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransferToAgentToolConfigOutput {
    #[serde(default)]
    pub transfers: Vec<AgentTransferOutput>,
}

impl TransferToAgentToolConfigOutput {
    pub fn builder() -> TransferToAgentToolConfigOutputBuilder {
        <TransferToAgentToolConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferToAgentToolConfigOutputBuilder {
    transfers: Option<Vec<AgentTransferOutput>>,
}

impl TransferToAgentToolConfigOutputBuilder {
    pub fn transfers(mut self, value: Vec<AgentTransferOutput>) -> Self {
        self.transfers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferToAgentToolConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfers`](TransferToAgentToolConfigOutputBuilder::transfers)
    pub fn build(self) -> Result<TransferToAgentToolConfigOutput, BuildError> {
        Ok(TransferToAgentToolConfigOutput {
            transfers: self
                .transfers
                .ok_or_else(|| BuildError::missing_field("transfers"))?,
        })
    }
}
