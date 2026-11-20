pub use crate::prelude::*;

/// Request for ConfirmUpdate (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfirmUpdateRequest {
    /// Also update the price on the service's future bookings.
    #[serde(skip)]
    pub reprice: Option<bool>,
    pub body: ReceptionConfirmServiceUpdateRequest,
}

impl ConfirmUpdateRequest {
    pub fn builder() -> ConfirmUpdateRequestBuilder {
        <ConfirmUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfirmUpdateRequestBuilder {
    reprice: Option<bool>,
    body: Option<ReceptionConfirmServiceUpdateRequest>,
}

impl ConfirmUpdateRequestBuilder {
    pub fn reprice(mut self, value: bool) -> Self {
        self.reprice = Some(value);
        self
    }

    pub fn body(mut self, value: ReceptionConfirmServiceUpdateRequest) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfirmUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](ConfirmUpdateRequestBuilder::body)
    pub fn build(self) -> Result<ConfirmUpdateRequest, BuildError> {
        Ok(ConfirmUpdateRequest {
            reprice: self.reprice,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
