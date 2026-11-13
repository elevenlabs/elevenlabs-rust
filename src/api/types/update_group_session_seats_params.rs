pub use crate::prelude::*;

/// Change the seat count of an existing group session registration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateGroupSessionSeatsParams {}

impl UpdateGroupSessionSeatsParams {
    pub fn builder() -> UpdateGroupSessionSeatsParamsBuilder {
        <UpdateGroupSessionSeatsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateGroupSessionSeatsParamsBuilder {}

impl UpdateGroupSessionSeatsParamsBuilder {
    /// Consumes the builder and constructs a [`UpdateGroupSessionSeatsParams`].
    pub fn build(self) -> Result<UpdateGroupSessionSeatsParams, BuildError> {
        Ok(UpdateGroupSessionSeatsParams {})
    }
}
