pub use crate::prelude::*;

/// Delete an existing staff member from the system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteStaffParams {}

impl DeleteStaffParams {
    pub fn builder() -> DeleteStaffParamsBuilder {
        <DeleteStaffParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteStaffParamsBuilder {}

impl DeleteStaffParamsBuilder {
    /// Consumes the builder and constructs a [`DeleteStaffParams`].
    pub fn build(self) -> Result<DeleteStaffParams, BuildError> {
        Ok(DeleteStaffParams {})
    }
}
