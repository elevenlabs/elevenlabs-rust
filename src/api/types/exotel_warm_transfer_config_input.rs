pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExotelWarmTransferConfigInput {
    pub transfer_destination: ExotelWarmTransferConfigInputTransferDestination,
}

impl ExotelWarmTransferConfigInput {
    pub fn builder() -> ExotelWarmTransferConfigInputBuilder {
        <ExotelWarmTransferConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExotelWarmTransferConfigInputBuilder {
    transfer_destination: Option<ExotelWarmTransferConfigInputTransferDestination>,
}

impl ExotelWarmTransferConfigInputBuilder {
    pub fn transfer_destination(
        mut self,
        value: ExotelWarmTransferConfigInputTransferDestination,
    ) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExotelWarmTransferConfigInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](ExotelWarmTransferConfigInputBuilder::transfer_destination)
    pub fn build(self) -> Result<ExotelWarmTransferConfigInput, BuildError> {
        Ok(ExotelWarmTransferConfigInput {
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
        })
    }
}
