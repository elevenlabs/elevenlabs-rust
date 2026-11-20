pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SipColdReferTransferConfigInput {
    /// Custom SIP headers to include when transferring the call. Each header can be either a static value or a dynamic variable reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_sip_headers: Option<Vec<SipColdReferTransferConfigInputCustomSipHeadersItem>>,
    pub transfer_destination: SipColdReferTransferConfigInputTransferDestination,
    /// User-to-User Information (RFC 7433) attached to the REFER. Carries call context such as CRM identifiers or escalation reason across the transfer boundary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uui: Option<UuiTransferConfig>,
    /// Play a ringing tone on the original call leg while the REFER completes. Disable if the receiving SBC/contact center should not hear it; the caller then hears silence until the transfer completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_dialtone: Option<bool>,
}

impl SipColdReferTransferConfigInput {
    pub fn builder() -> SipColdReferTransferConfigInputBuilder {
        <SipColdReferTransferConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipColdReferTransferConfigInputBuilder {
    custom_sip_headers: Option<Vec<SipColdReferTransferConfigInputCustomSipHeadersItem>>,
    transfer_destination: Option<SipColdReferTransferConfigInputTransferDestination>,
    uui: Option<UuiTransferConfig>,
    play_dialtone: Option<bool>,
}

impl SipColdReferTransferConfigInputBuilder {
    pub fn custom_sip_headers(
        mut self,
        value: Vec<SipColdReferTransferConfigInputCustomSipHeadersItem>,
    ) -> Self {
        self.custom_sip_headers = Some(value);
        self
    }

    pub fn transfer_destination(
        mut self,
        value: SipColdReferTransferConfigInputTransferDestination,
    ) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    pub fn uui(mut self, value: UuiTransferConfig) -> Self {
        self.uui = Some(value);
        self
    }

    pub fn play_dialtone(mut self, value: bool) -> Self {
        self.play_dialtone = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SipColdReferTransferConfigInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](SipColdReferTransferConfigInputBuilder::transfer_destination)
    pub fn build(self) -> Result<SipColdReferTransferConfigInput, BuildError> {
        Ok(SipColdReferTransferConfigInput {
            custom_sip_headers: self.custom_sip_headers,
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
            uui: self.uui,
            play_dialtone: self.play_dialtone,
        })
    }
}
