pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SipColdConferenceTransferConfigInput {
    pub transfer_destination: SipColdConferenceTransferConfigInputTransferDestination,
}

impl SipColdConferenceTransferConfigInput {
    pub fn builder() -> SipColdConferenceTransferConfigInputBuilder {
        <SipColdConferenceTransferConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipColdConferenceTransferConfigInputBuilder {
    transfer_destination: Option<SipColdConferenceTransferConfigInputTransferDestination>,
}

impl SipColdConferenceTransferConfigInputBuilder {
    pub fn transfer_destination(
        mut self,
        value: SipColdConferenceTransferConfigInputTransferDestination,
    ) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SipColdConferenceTransferConfigInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](SipColdConferenceTransferConfigInputBuilder::transfer_destination)
    pub fn build(self) -> Result<SipColdConferenceTransferConfigInput, BuildError> {
        Ok(SipColdConferenceTransferConfigInput {
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
        })
    }
}
