pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferToNumberToolConfigOutput {
    #[serde(default)]
    pub transfers: Vec<PhoneNumberTransfer>,
    /// Per-provider transfer configuration. Mutually exclusive with `transfers`: a tool sets exactly one of the two. When present, the configuration matching the call's telephony provider is offered to the agent and executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_rules: Option<Vec<TransferRuleOutput>>,
    /// Whether to play a message to the client while they wait for transfer. Defaults to true for backward compatibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_client_message: Option<bool>,
}

impl TransferToNumberToolConfigOutput {
    pub fn builder() -> TransferToNumberToolConfigOutputBuilder {
        <TransferToNumberToolConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferToNumberToolConfigOutputBuilder {
    transfers: Option<Vec<PhoneNumberTransfer>>,
    transfer_rules: Option<Vec<TransferRuleOutput>>,
    enable_client_message: Option<bool>,
}

impl TransferToNumberToolConfigOutputBuilder {
    pub fn transfers(mut self, value: Vec<PhoneNumberTransfer>) -> Self {
        self.transfers = Some(value);
        self
    }

    pub fn transfer_rules(mut self, value: Vec<TransferRuleOutput>) -> Self {
        self.transfer_rules = Some(value);
        self
    }

    pub fn enable_client_message(mut self, value: bool) -> Self {
        self.enable_client_message = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferToNumberToolConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfers`](TransferToNumberToolConfigOutputBuilder::transfers)
    pub fn build(self) -> Result<TransferToNumberToolConfigOutput, BuildError> {
        Ok(TransferToNumberToolConfigOutput {
            transfers: self
                .transfers
                .ok_or_else(|| BuildError::missing_field("transfers"))?,
            transfer_rules: self.transfer_rules,
            enable_client_message: self.enable_client_message,
        })
    }
}
