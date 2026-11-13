pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAssetParams {
    /// Whether to offer an available_hours timezone, set from the workspace's enable_per_resource_timezones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_resource_timezones: Option<bool>,
}

impl CreateAssetParams {
    pub fn builder() -> CreateAssetParamsBuilder {
        <CreateAssetParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAssetParamsBuilder {
    per_resource_timezones: Option<bool>,
}

impl CreateAssetParamsBuilder {
    pub fn per_resource_timezones(mut self, value: bool) -> Self {
        self.per_resource_timezones = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAssetParams`].
    pub fn build(self) -> Result<CreateAssetParams, BuildError> {
        Ok(CreateAssetParams {
            per_resource_timezones: self.per_resource_timezones,
        })
    }
}
