pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateClientAppointmentParams {}

impl CreateClientAppointmentParams {
    pub fn builder() -> CreateClientAppointmentParamsBuilder {
        <CreateClientAppointmentParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateClientAppointmentParamsBuilder {}

impl CreateClientAppointmentParamsBuilder {
    /// Consumes the builder and constructs a [`CreateClientAppointmentParams`].
    pub fn build(self) -> Result<CreateClientAppointmentParams, BuildError> {
        Ok(CreateClientAppointmentParams {})
    }
}
