pub use crate::prelude::*;

/// Update an existing staff member's information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateStaffParams {
    /// Whether to offer a working_hours timezone, set from the workspace's enable_per_resource_timezones. While that is off every staff member uses the workspace timezone, so the key is not offered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_resource_timezones: Option<bool>,
}

impl UpdateStaffParams {
    pub fn builder() -> UpdateStaffParamsBuilder {
        <UpdateStaffParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStaffParamsBuilder {
    per_resource_timezones: Option<bool>,
}

impl UpdateStaffParamsBuilder {
    pub fn per_resource_timezones(mut self, value: bool) -> Self {
        self.per_resource_timezones = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateStaffParams`].
    pub fn build(self) -> Result<UpdateStaffParams, BuildError> {
        Ok(UpdateStaffParams {
            per_resource_timezones: self.per_resource_timezones,
        })
    }
}
