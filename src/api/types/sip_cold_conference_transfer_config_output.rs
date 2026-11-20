pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SipColdConferenceTransferConfigOutput {
    pub transfer_destination: SipColdConferenceTransferConfigOutputTransferDestination,
}

impl SipColdConferenceTransferConfigOutput {
    pub fn builder() -> SipColdConferenceTransferConfigOutputBuilder {
        <SipColdConferenceTransferConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipColdConferenceTransferConfigOutputBuilder {
    transfer_destination: Option<SipColdConferenceTransferConfigOutputTransferDestination>,
}

impl SipColdConferenceTransferConfigOutputBuilder {
    pub fn transfer_destination(
        mut self,
        value: SipColdConferenceTransferConfigOutputTransferDestination,
    ) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SipColdConferenceTransferConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](SipColdConferenceTransferConfigOutputBuilder::transfer_destination)
    pub fn build(self) -> Result<SipColdConferenceTransferConfigOutput, BuildError> {
        Ok(SipColdConferenceTransferConfigOutput {
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
        })
    }
}
