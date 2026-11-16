pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStaffParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_kwargs: Option<serde_json::Value>,
}

impl ListStaffParams {
    pub fn builder() -> ListStaffParamsBuilder {
        <ListStaffParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStaffParamsBuilder {
    list_kwargs: Option<serde_json::Value>,
}

impl ListStaffParamsBuilder {
    pub fn list_kwargs(mut self, value: serde_json::Value) -> Self {
        self.list_kwargs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListStaffParams`].
    pub fn build(self) -> Result<ListStaffParams, BuildError> {
        Ok(ListStaffParams {
            list_kwargs: self.list_kwargs,
        })
    }
}
