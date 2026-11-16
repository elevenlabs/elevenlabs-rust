pub use crate::prelude::*;

/// Every owner-editable field, including what callers must not hear.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OwnerListServicesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_kwargs: Option<serde_json::Value>,
}

impl OwnerListServicesParams {
    pub fn builder() -> OwnerListServicesParamsBuilder {
        <OwnerListServicesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnerListServicesParamsBuilder {
    list_kwargs: Option<serde_json::Value>,
}

impl OwnerListServicesParamsBuilder {
    pub fn list_kwargs(mut self, value: serde_json::Value) -> Self {
        self.list_kwargs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OwnerListServicesParams`].
    pub fn build(self) -> Result<OwnerListServicesParams, BuildError> {
        Ok(OwnerListServicesParams {
            list_kwargs: self.list_kwargs,
        })
    }
}
