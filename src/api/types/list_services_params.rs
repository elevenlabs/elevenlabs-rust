pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListServicesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_kwargs: Option<serde_json::Value>,
}

impl ListServicesParams {
    pub fn builder() -> ListServicesParamsBuilder {
        <ListServicesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListServicesParamsBuilder {
    list_kwargs: Option<serde_json::Value>,
}

impl ListServicesParamsBuilder {
    pub fn list_kwargs(mut self, value: serde_json::Value) -> Self {
        self.list_kwargs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListServicesParams`].
    pub fn build(self) -> Result<ListServicesParams, BuildError> {
        Ok(ListServicesParams {
            list_kwargs: self.list_kwargs,
        })
    }
}
