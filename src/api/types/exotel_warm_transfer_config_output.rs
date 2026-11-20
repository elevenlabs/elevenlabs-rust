pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExotelWarmTransferConfigOutput {
    pub transfer_destination: ExotelWarmTransferConfigOutputTransferDestination,
}

impl ExotelWarmTransferConfigOutput {
    pub fn builder() -> ExotelWarmTransferConfigOutputBuilder {
        <ExotelWarmTransferConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExotelWarmTransferConfigOutputBuilder {
    transfer_destination: Option<ExotelWarmTransferConfigOutputTransferDestination>,
}

impl ExotelWarmTransferConfigOutputBuilder {
    pub fn transfer_destination(
        mut self,
        value: ExotelWarmTransferConfigOutputTransferDestination,
    ) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExotelWarmTransferConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](ExotelWarmTransferConfigOutputBuilder::transfer_destination)
    pub fn build(self) -> Result<ExotelWarmTransferConfigOutput, BuildError> {
        Ok(ExotelWarmTransferConfigOutput {
            transfer_destination: self
                .transfer_destination
                .ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
        })
    }
}
