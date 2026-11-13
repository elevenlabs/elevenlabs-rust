pub use crate::prelude::*;

/// Every owner-editable field, available hours included.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OwnerListAssetsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_kwargs: Option<serde_json::Value>,
}

impl OwnerListAssetsParams {
    pub fn builder() -> OwnerListAssetsParamsBuilder {
        <OwnerListAssetsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnerListAssetsParamsBuilder {
    list_kwargs: Option<serde_json::Value>,
}

impl OwnerListAssetsParamsBuilder {
    pub fn list_kwargs(mut self, value: serde_json::Value) -> Self {
        self.list_kwargs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OwnerListAssetsParams`].
    pub fn build(self) -> Result<OwnerListAssetsParams, BuildError> {
        Ok(OwnerListAssetsParams {
            list_kwargs: self.list_kwargs,
        })
    }
}
