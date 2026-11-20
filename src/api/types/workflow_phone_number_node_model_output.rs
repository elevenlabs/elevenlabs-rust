pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowPhoneNumberNodeModelOutput {
    /// Custom SIP headers to include when transferring the call. Each header can be either a static value or a dynamic variable reference.
    #[serde(default)]
    pub custom_sip_headers: Vec<WorkflowPhoneNumberNodeModelOutputCustomSipHeadersItem>,
    pub transfer_destination: WorkflowPhoneNumberNodeModelOutputTransferDestination,
    pub transfer_type: TransferTypeEnum,
    /// When True, a ringing tone is played on the original call leg while a SIP REFER transfer completes. The tone is carried over RTP to the SIP peer executing the REFER, so disable this if the receiving system (e.g. an SBC or contact center) should not hear it. When disabled the caller hears silence until the transfer completes. SIP REFER transfers only.
    #[serde(default)]
    pub sip_refer_play_dialtone: bool,
    /// User-to-User Information (RFC 7433) to attach to SIP REFER transfers. Carries call context such as CRM identifiers or escalation reason across the transfer boundary.
    pub uui: Option<UuiTransferConfig>,
    /// DTMF digits to send after call connects (e.g., 'ww1234' for extension). Can be either a static value or a dynamic variable reference. Use 'w' for 0.5s pause. Only supported for Twilio transfers.
    pub post_dial_digits: Option<WorkflowPhoneNumberNodeModelOutputPostDialDigits>,
    pub position: PositionOutput,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(default)]
    pub edge_order: Vec<String>,
    /// Per-provider transfer configuration. Mutually exclusive with a non-empty `transfer_destination`: when present, the configuration matching the call's telephony provider is executed and the legacy transfer fields are ignored.
    #[serde(default)]
    pub provider_configs: Vec<WorkflowPhoneNumberNodeModelOutputProviderConfigsItem>,
}

impl WorkflowPhoneNumberNodeModelOutput {
    pub fn builder() -> WorkflowPhoneNumberNodeModelOutputBuilder {
        <WorkflowPhoneNumberNodeModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowPhoneNumberNodeModelOutputBuilder {
    custom_sip_headers: Option<Vec<WorkflowPhoneNumberNodeModelOutputCustomSipHeadersItem>>,
    transfer_destination: Option<WorkflowPhoneNumberNodeModelOutputTransferDestination>,
    transfer_type: Option<TransferTypeEnum>,
    sip_refer_play_dialtone: Option<bool>,
    uui: Option<UuiTransferConfig>,
    post_dial_digits: Option<WorkflowPhoneNumberNodeModelOutputPostDialDigits>,
    position: Option<PositionOutput>,
    edge_order: Option<Vec<String>>,
    provider_configs: Option<Vec<WorkflowPhoneNumberNodeModelOutputProviderConfigsItem>>,
}

impl WorkflowPhoneNumberNodeModelOutputBuilder {
    pub fn custom_sip_headers(
        mut self,
        value: Vec<WorkflowPhoneNumberNodeModelOutputCustomSipHeadersItem>,
    ) -> Self {
        self.custom_sip_headers = Some(value);
        self
    }

    pub fn transfer_destination(
        mut self,
        value: WorkflowPhoneNumberNodeModelOutputTransferDestination,
    ) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    pub fn transfer_type(mut self, value: TransferTypeEnum) -> Self {
        self.transfer_type = Some(value);
        self
    }

    pub fn sip_refer_play_dialtone(mut self, value: bool) -> Self {
        self.sip_refer_play_dialtone = Some(value);
        self
    }

    pub fn uui(mut self, value: UuiTransferConfig) -> Self {
        self.uui = Some(value);
        self
    }

    pub fn post_dial_digits(
        mut self,
        value: WorkflowPhoneNumberNodeModelOutputPostDialDigits,
    ) -> Self {
        self.post_dial_digits = Some(value);
        self
    }

    pub fn position(mut self, value: PositionOutput) -> Self {
        self.position = Some(value);
        self
    }

    pub fn edge_order(mut self, value: Vec<String>) -> Self {
        self.edge_order = Some(value);
        self
    }

    pub fn provider_configs(
        mut self,
        value: Vec<WorkflowPhoneNumberNodeModelOutputProviderConfigsItem>,
    ) -> Self {
        self.provider_configs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowPhoneNumberNodeModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`custom_sip_headers`](WorkflowPhoneNumberNodeModelOutputBuilder::custom_sip_headers)
    /// - [`transfer_destination`](WorkflowPhoneNumberNodeModelOutputBuilder::transfer_destination)
    /// - [`transfer_type`](WorkflowPhoneNumberNodeModelOutputBuilder::transfer_type)
    /// - [`sip_refer_play_dialtone`](WorkflowPhoneNumberNodeModelOutputBuilder::sip_refer_play_dialtone)
    /// - [`position`](WorkflowPhoneNumberNodeModelOutputBuilder::position)
    /// - [`edge_order`](WorkflowPhoneNumberNodeModelOutputBuilder::edge_order)
    /// - [`provider_configs`](WorkflowPhoneNumberNodeModelOutputBuilder::provider_configs)
    pub fn build(self) -> Result<WorkflowPhoneNumberNodeModelOutput, BuildError> {
        Ok(WorkflowPhoneNumberNodeModelOutput {
            custom_sip_headers: self
                .custom_sip_headers
                .ok_or_else(|| BuildError::missing_field("custom_sip_headers"))?,
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
            transfer_type: self
                .transfer_type
                .ok_or_else(|| BuildError::missing_field("transfer_type"))?,
            sip_refer_play_dialtone: self
                .sip_refer_play_dialtone
                .ok_or_else(|| BuildError::missing_field("sip_refer_play_dialtone"))?,
            uui: self.uui,
            post_dial_digits: self.post_dial_digits,
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            edge_order: self
                .edge_order
                .ok_or_else(|| BuildError::missing_field("edge_order"))?,
            provider_configs: self
                .provider_configs
                .ok_or_else(|| BuildError::missing_field("provider_configs"))?,
        })
    }
}
