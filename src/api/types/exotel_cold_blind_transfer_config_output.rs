pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExotelColdBlindTransferConfigOutput {
    pub transfer_destination: ExotelColdBlindTransferConfigOutputTransferDestination,
}

impl ExotelColdBlindTransferConfigOutput {
    pub fn builder() -> ExotelColdBlindTransferConfigOutputBuilder {
        <ExotelColdBlindTransferConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExotelColdBlindTransferConfigOutputBuilder {
    transfer_destination: Option<ExotelColdBlindTransferConfigOutputTransferDestination>,
}

impl ExotelColdBlindTransferConfigOutputBuilder {
    pub fn transfer_destination(
        mut self,
        value: ExotelColdBlindTransferConfigOutputTransferDestination,
    ) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExotelColdBlindTransferConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](ExotelColdBlindTransferConfigOutputBuilder::transfer_destination)
    pub fn build(self) -> Result<ExotelColdBlindTransferConfigOutput, BuildError> {
        Ok(ExotelColdBlindTransferConfigOutput {
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
        })
    }
}
