pub use crate::prelude::*;

/// Every owner-editable field, including what callers must not hear.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OwnerListProductsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_kwargs: Option<serde_json::Value>,
}

impl OwnerListProductsParams {
    pub fn builder() -> OwnerListProductsParamsBuilder {
        <OwnerListProductsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnerListProductsParamsBuilder {
    list_kwargs: Option<serde_json::Value>,
}

impl OwnerListProductsParamsBuilder {
    pub fn list_kwargs(mut self, value: serde_json::Value) -> Self {
        self.list_kwargs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OwnerListProductsParams`].
    pub fn build(self) -> Result<OwnerListProductsParams, BuildError> {
        Ok(OwnerListProductsParams {
            list_kwargs: self.list_kwargs,
        })
    }
}
