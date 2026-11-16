pub use crate::prelude::*;

/// Look up an appointment by the booking confirmation number the caller quotes.
///
/// The confirmation number is the 8-character code shown on the booking
/// confirmation page after a ``#``. Callers may read it back with or
/// without the leading ``#`` and with varied spacing; the tool normalizes
/// the input and does a prefix match on the stored calendar item id.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAppointmentByConfirmationNumberParams {}

impl GetAppointmentByConfirmationNumberParams {
    pub fn builder() -> GetAppointmentByConfirmationNumberParamsBuilder {
        <GetAppointmentByConfirmationNumberParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAppointmentByConfirmationNumberParamsBuilder {}

impl GetAppointmentByConfirmationNumberParamsBuilder {
    /// Consumes the builder and constructs a [`GetAppointmentByConfirmationNumberParams`].
    pub fn build(self) -> Result<GetAppointmentByConfirmationNumberParams, BuildError> {
        Ok(GetAppointmentByConfirmationNumberParams {})
    }
}
