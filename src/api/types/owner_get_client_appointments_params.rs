pub use crate::prelude::*;

/// Appointments with the booking channel, internal notes and intake answers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OwnerGetClientAppointmentsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cancelled: Option<bool>,
}

impl OwnerGetClientAppointmentsParams {
    pub fn builder() -> OwnerGetClientAppointmentsParamsBuilder {
        <OwnerGetClientAppointmentsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnerGetClientAppointmentsParamsBuilder {
    include_cancelled: Option<bool>,
}

impl OwnerGetClientAppointmentsParamsBuilder {
    pub fn include_cancelled(mut self, value: bool) -> Self {
        self.include_cancelled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OwnerGetClientAppointmentsParams`].
    pub fn build(self) -> Result<OwnerGetClientAppointmentsParams, BuildError> {
        Ok(OwnerGetClientAppointmentsParams {
            include_cancelled: self.include_cancelled,
        })
    }
}
