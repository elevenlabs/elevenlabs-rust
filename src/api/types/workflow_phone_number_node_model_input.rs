pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowPhoneNumberNodeModelInput {
    /// Custom SIP headers to include when transferring the call. Each header can be either a static value or a dynamic variable reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_sip_headers: Option<Vec<WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem>>,
    pub transfer_destination: WorkflowPhoneNumberNodeModelInputTransferDestination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<TransferTypeEnum>,
    /// When True, a ringing tone is played on the original call leg while a SIP REFER transfer completes. The tone is carried over RTP to the SIP peer executing the REFER, so disable this if the receiving system (e.g. an SBC or contact center) should not hear it. When disabled the caller hears silence until the transfer completes. SIP REFER transfers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip_refer_play_dialtone: Option<bool>,
    /// User-to-User Information (RFC 7433) to attach to SIP REFER transfers. Carries call context such as CRM identifiers or escalation reason across the transfer boundary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uui: Option<UuiTransferConfig>,
    /// DTMF digits to send after call connects (e.g., 'ww1234' for extension). Can be either a static value or a dynamic variable reference. Use 'w' for 0.5s pause. Only supported for Twilio transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_dial_digits: Option<WorkflowPhoneNumberNodeModelInputPostDialDigits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionInput>,
    /// The ids of outgoing edges in the order they should be evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_order: Option<Vec<String>>,
}

impl WorkflowPhoneNumberNodeModelInput {
    pub fn builder() -> WorkflowPhoneNumberNodeModelInputBuilder {
        <WorkflowPhoneNumberNodeModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowPhoneNumberNodeModelInputBuilder {
    custom_sip_headers: Option<Vec<WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem>>,
    transfer_destination: Option<WorkflowPhoneNumberNodeModelInputTransferDestination>,
    transfer_type: Option<TransferTypeEnum>,
    sip_refer_play_dialtone: Option<bool>,
    uui: Option<UuiTransferConfig>,
    post_dial_digits: Option<WorkflowPhoneNumberNodeModelInputPostDialDigits>,
    position: Option<PositionInput>,
    edge_order: Option<Vec<String>>,
}

impl WorkflowPhoneNumberNodeModelInputBuilder {
    pub fn custom_sip_headers(
        mut self,
        value: Vec<WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem>,
    ) -> Self {
        self.custom_sip_headers = Some(value);
        self
    }

    pub fn transfer_destination(
        mut self,
        value: WorkflowPhoneNumberNodeModelInputTransferDestination,
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
        value: WorkflowPhoneNumberNodeModelInputPostDialDigits,
    ) -> Self {
        self.post_dial_digits = Some(value);
        self
    }

    pub fn position(mut self, value: PositionInput) -> Self {
        self.position = Some(value);
        self
    }

    pub fn edge_order(mut self, value: Vec<String>) -> Self {
        self.edge_order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowPhoneNumberNodeModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](WorkflowPhoneNumberNodeModelInputBuilder::transfer_destination)
    pub fn build(self) -> Result<WorkflowPhoneNumberNodeModelInput, BuildError> {
        Ok(WorkflowPhoneNumberNodeModelInput {
            custom_sip_headers: self.custom_sip_headers,
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
            transfer_type: self.transfer_type,
            sip_refer_play_dialtone: self.sip_refer_play_dialtone,
            uui: self.uui,
            post_dial_digits: self.post_dial_digits,
            position: self.position,
            edge_order: self.edge_order,
        })
    }
}
