pub use crate::prelude::*;

/// Every owner-editable field, working hours included.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OwnerListStaffParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_kwargs: Option<serde_json::Value>,
}

impl OwnerListStaffParams {
    pub fn builder() -> OwnerListStaffParamsBuilder {
        <OwnerListStaffParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnerListStaffParamsBuilder {
    list_kwargs: Option<serde_json::Value>,
}

impl OwnerListStaffParamsBuilder {
    pub fn list_kwargs(mut self, value: serde_json::Value) -> Self {
        self.list_kwargs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OwnerListStaffParams`].
    pub fn build(self) -> Result<OwnerListStaffParams, BuildError> {
        Ok(OwnerListStaffParams {
            list_kwargs: self.list_kwargs,
        })
    }
}
