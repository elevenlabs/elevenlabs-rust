pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SipColdReferTransferConfigOutput {
    /// Custom SIP headers to include when transferring the call. Each header can be either a static value or a dynamic variable reference.
    #[serde(default)]
    pub custom_sip_headers: Vec<SipColdReferTransferConfigOutputCustomSipHeadersItem>,
    pub transfer_destination: SipColdReferTransferConfigOutputTransferDestination,
    /// User-to-User Information (RFC 7433) attached to the REFER. Carries call context such as CRM identifiers or escalation reason across the transfer boundary.
    pub uui: Option<UuiTransferConfig>,
    /// Play a ringing tone on the original call leg while the REFER completes. Disable if the receiving SBC/contact center should not hear it; the caller then hears silence until the transfer completes.
    #[serde(default)]
    pub play_dialtone: bool,
}

impl SipColdReferTransferConfigOutput {
    pub fn builder() -> SipColdReferTransferConfigOutputBuilder {
        <SipColdReferTransferConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipColdReferTransferConfigOutputBuilder {
    custom_sip_headers: Option<Vec<SipColdReferTransferConfigOutputCustomSipHeadersItem>>,
    transfer_destination: Option<SipColdReferTransferConfigOutputTransferDestination>,
    uui: Option<UuiTransferConfig>,
    play_dialtone: Option<bool>,
}

impl SipColdReferTransferConfigOutputBuilder {
    pub fn custom_sip_headers(
        mut self,
        value: Vec<SipColdReferTransferConfigOutputCustomSipHeadersItem>,
    ) -> Self {
        self.custom_sip_headers = Some(value);
        self
    }

    pub fn transfer_destination(
        mut self,
        value: SipColdReferTransferConfigOutputTransferDestination,
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

    /// Consumes the builder and constructs a [`SipColdReferTransferConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`custom_sip_headers`](SipColdReferTransferConfigOutputBuilder::custom_sip_headers)
    /// - [`transfer_destination`](SipColdReferTransferConfigOutputBuilder::transfer_destination)
    /// - [`play_dialtone`](SipColdReferTransferConfigOutputBuilder::play_dialtone)
    pub fn build(self) -> Result<SipColdReferTransferConfigOutput, BuildError> {
        Ok(SipColdReferTransferConfigOutput {
            custom_sip_headers: self
                .custom_sip_headers
                .ok_or_else(|| BuildError::missing_field("custom_sip_headers"))?,
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
            uui: self.uui,
            play_dialtone: self
                .play_dialtone
                .ok_or_else(|| BuildError::missing_field("play_dialtone"))?,
        })
    }
}
