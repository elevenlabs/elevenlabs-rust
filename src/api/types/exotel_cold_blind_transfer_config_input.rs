pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExotelColdBlindTransferConfigInput {
    pub transfer_destination: ExotelColdBlindTransferConfigInputTransferDestination,
}

impl ExotelColdBlindTransferConfigInput {
    pub fn builder() -> ExotelColdBlindTransferConfigInputBuilder {
        <ExotelColdBlindTransferConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExotelColdBlindTransferConfigInputBuilder {
    transfer_destination: Option<ExotelColdBlindTransferConfigInputTransferDestination>,
}

impl ExotelColdBlindTransferConfigInputBuilder {
    pub fn transfer_destination(
        mut self,
        value: ExotelColdBlindTransferConfigInputTransferDestination,
    ) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExotelColdBlindTransferConfigInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](ExotelColdBlindTransferConfigInputBuilder::transfer_destination)
    pub fn build(self) -> Result<ExotelColdBlindTransferConfigInput, BuildError> {
        Ok(ExotelColdBlindTransferConfigInput {
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
        })
    }
}
