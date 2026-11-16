pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAssetParams {
    /// Whether to offer an available_hours timezone, set from the workspace's enable_per_resource_timezones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_resource_timezones: Option<bool>,
}

impl UpdateAssetParams {
    pub fn builder() -> UpdateAssetParamsBuilder {
        <UpdateAssetParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAssetParamsBuilder {
    per_resource_timezones: Option<bool>,
}

impl UpdateAssetParamsBuilder {
    pub fn per_resource_timezones(mut self, value: bool) -> Self {
        self.per_resource_timezones = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAssetParams`].
    pub fn build(self) -> Result<UpdateAssetParams, BuildError> {
        Ok(UpdateAssetParams {
            per_resource_timezones: self.per_resource_timezones,
        })
    }
}
